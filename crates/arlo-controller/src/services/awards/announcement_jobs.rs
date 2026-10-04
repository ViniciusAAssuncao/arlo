use super::global_player_evidence::{
    load_dated_global_player_evidence, DatedSeasonSource, GlobalSeasonSource,
};
use super::season_age_cutoff::season_start_date;
use crate::domain::calendar::{CalendarDate, CalendarSystem, ResolvedCalendarDate};
use crate::error::{ControllerError, ControllerResult};
use crate::services::calendar::date_encoder;
use arlo_awards::{resolve_award, resolve_roster_award, AwardError};
use arlo_domain::{
    AwardDefinition, AwardEvaluationWindow, AwardInstanceContext, AwardOrganizerPolicy,
    AwardRecipientKind, AwardResultKind, AwardScopeKind, AwardTrigger,
};
use arlo_persistence::repositories::{award_repository, award_roster_repository};
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(FromRow)]
struct AnnouncementJob {
    award_definition_id: String,
    announcement_year: i64,
    scope_id: String,
}

#[derive(FromRow)]
struct SeasonRow {
    id: String,
    competition_id: String,
    reference_year: i64,
    status: String,
}

pub(crate) async fn process_announcement_jobs(
    pool: &SqlitePool,
    calendar: &CalendarSystem,
    date: CalendarDate,
) -> ControllerResult<u32> {
    let definitions = arlo_catalog::list_active_awards(pool).await?;
    enqueue(pool, calendar, date, &definitions).await?;
    let jobs = sqlx::query_as::<_, AnnouncementJob>(
        "SELECT award_definition_id, announcement_year, scope_id FROM award_announcement_jobs WHERE status = 'Pending' ORDER BY announcement_year, award_definition_id, scope_id",
    )
    .fetch_all(pool)
    .await?;
    let mut completed = 0;
    for job in jobs {
        let definition_id = Uuid::parse_str(&job.award_definition_id)?;
        let Some(definition) = definitions.iter().find(|item| item.id == definition_id) else {
            set_status(
                pool,
                &job,
                "Pending",
                "Award definition is inactive or missing",
            )
            .await?;
            continue;
        };
        if !is_supported(definition) {
            set_status(
                pool,
                &job,
                "Pending",
                "Unsupported announcement award definition",
            )
            .await?;
            continue;
        }
        let cutoff = announcement_date(calendar, definition, job.announcement_year)?;
        if cutoff > date {
            continue;
        }
        let seasons = sqlx::query_as::<_, SeasonRow>(
            "SELECT id, competition_id, reference_year, status FROM season_instances WHERE reference_year IN (?, ?) ORDER BY reference_year, competition_id, id",
        )
        .bind(job.announcement_year - 1)
        .bind(job.announcement_year)
        .fetch_all(pool)
        .await?;
        let scope_id = if job.scope_id.is_empty() {
            None
        } else {
            Some(Uuid::parse_str(&job.scope_id)?)
        };
        let mut sources = Vec::new();
        for prior in seasons
            .iter()
            .filter(|item| item.reference_year == job.announcement_year - 1)
        {
            let competition_id = Uuid::parse_str(&prior.competition_id)?;
            if scope_id.is_some_and(|id| id != competition_id)
                || (!definition.eligible_competitions.is_empty()
                    && !definition.eligible_competitions.contains(&competition_id))
            {
                continue;
            }
            if prior.status != "Completed" {
                sources.clear();
                break;
            }
            let prior_id = Uuid::parse_str(&prior.id)?;
            let start = season_start_date(pool, prior_id).await?;
            sources.push(DatedSeasonSource {
                source: GlobalSeasonSource {
                    season_id: prior_id,
                    competition_id,
                },
                start,
                end: cutoff,
                age_cutoff_season_id: prior_id,
                include_honors: true,
            });
            for current in seasons.iter().filter(|item| {
                item.reference_year == job.announcement_year
                    && item.competition_id == prior.competition_id
            }) {
                sources.push(DatedSeasonSource {
                    source: GlobalSeasonSource {
                        season_id: Uuid::parse_str(&current.id)?,
                        competition_id,
                    },
                    start,
                    end: cutoff,
                    age_cutoff_season_id: prior_id,
                    include_honors: false,
                });
            }
        }
        if sources.is_empty()
            || (!definition.eligible_competitions.is_empty()
                && definition.eligible_competitions.iter().any(|id| {
                    !sources
                        .iter()
                        .any(|source| source.source.competition_id == *id)
                }))
        {
            set_status(
                pool,
                &job,
                "Pending",
                "Waiting for eligible previous seasons",
            )
            .await?;
            continue;
        }
        if !sources_ready(pool, definition, &sources).await? {
            set_status(pool, &job, "Pending", "Waiting for previous season honors").await?;
            continue;
        }
        let evidence = load_dated_global_player_evidence(pool, &sources).await?;
        if !criteria_available(definition, &evidence) {
            set_status(
                pool,
                &job,
                "Pending",
                "Award criteria require unavailable metrics",
            )
            .await?;
            continue;
        }
        let context = AwardInstanceContext {
            period_key: format!(
                "announcement:{}:{}",
                job.announcement_year,
                cutoff.day_of_year()
            ),
            scope_id,
            selection_model_version: 1,
        };
        let seed = job
            .announcement_year
            .to_le_bytes()
            .iter()
            .chain(definition.id.as_bytes())
            .chain(job.scope_id.as_bytes())
            .fold(0xcbf29ce484222325_u64, |value, byte| {
                value.wrapping_mul(0x100000001b3) ^ u64::from(*byte)
            });
        let organizer_league_id =
            if definition.organizer_policy == AwardOrganizerPolicy::LeagueCommittee {
                if let Some(competition_id) = scope_id {
                    sqlx::query_scalar::<_, i64>("SELECT 1 FROM leagues WHERE competition_id = ?")
                        .bind(competition_id.to_string())
                        .fetch_optional(pool)
                        .await?
                        .map(|_| competition_id)
                } else {
                    None
                }
            } else {
                None
            };
        let mut tx = pool.begin().await?;
        let persisted = match definition.result_kind {
            AwardResultKind::SingleWinner => resolve_award(definition, &context, &evidence, seed)
                .map(|resolution| (Some(resolution), None)),
            AwardResultKind::Roster => resolve_roster_award(definition, &context, &evidence, seed)
                .map(|resolution| (None, Some(resolution))),
        };
        let (single, roster) = match persisted {
            Ok(value) => value,
            Err(AwardError::NoEligibleCandidates) => {
                drop(tx);
                set_status(pool, &job, "Unavailable", "No eligible candidates").await?;
                continue;
            }
            Err(error) => {
                drop(tx);
                set_status(pool, &job, "Pending", &error.to_string()).await?;
                continue;
            }
        };
        let instance_id = if let Some(resolution) = single {
            award_repository::persist_resolution(
                &mut tx,
                &resolution,
                definition,
                organizer_league_id,
            )
            .await?
        } else if let Some(resolution) = roster {
            award_roster_repository::persist_roster_resolution(
                &mut tx,
                &resolution,
                definition,
                organizer_league_id,
            )
            .await?
        } else {
            return Err(ControllerError::InvalidData(
                "Missing award resolution".into(),
            ));
        };
        for source in &sources {
            sqlx::query("INSERT OR IGNORE INTO award_announcement_sources (award_instance_id, season_instance_id, competition_id, start_year, start_day_of_year, end_year, end_day_of_year) VALUES (?, ?, ?, ?, ?, ?, ?)")
                .bind(instance_id.to_string())
                .bind(source.source.season_id.to_string())
                .bind(source.source.competition_id.to_string())
                .bind(source.start.year())
                .bind(i64::from(source.start.day_of_year()))
                .bind(source.end.year())
                .bind(i64::from(source.end.day_of_year()))
                .execute(&mut *tx).await?;
        }
        sqlx::query("UPDATE award_announcement_jobs SET status = 'Completed', reason = NULL, finished_at = CURRENT_TIMESTAMP WHERE award_definition_id = ? AND announcement_year = ? AND scope_id = ?")
            .bind(&job.award_definition_id).bind(job.announcement_year).bind(&job.scope_id)
            .execute(&mut *tx).await?;
        tx.commit().await?;
        completed += 1;
    }
    Ok(completed)
}

async fn enqueue(
    pool: &SqlitePool,
    calendar: &CalendarSystem,
    date: CalendarDate,
    definitions: &[AwardDefinition],
) -> ControllerResult<()> {
    for definition in definitions.iter().filter(|item| is_supported(item)) {
        if announcement_date(calendar, definition, date.year())? != date {
            continue;
        }
        if definition.scope == AwardScopeKind::Global {
            insert_job(pool, definition.id, date.year(), "").await?;
        } else {
            let rows = sqlx::query_scalar::<_, String>(
                "SELECT DISTINCT competition_id FROM season_instances WHERE reference_year = ?",
            )
            .bind(date.year() - 1)
            .fetch_all(pool)
            .await?;
            for competition in rows {
                let id = Uuid::parse_str(&competition)?;
                if definition.eligible_competitions.is_empty()
                    || definition.eligible_competitions.contains(&id)
                {
                    insert_job(pool, definition.id, date.year(), &competition).await?;
                }
            }
        }
    }
    Ok(())
}

async fn insert_job(pool: &SqlitePool, id: Uuid, year: i64, scope: &str) -> ControllerResult<()> {
    sqlx::query("INSERT INTO award_announcement_jobs (award_definition_id, announcement_year, scope_id) VALUES (?, ?, ?) ON CONFLICT DO NOTHING")
        .bind(id.to_string()).bind(year).bind(scope).execute(pool).await?;
    Ok(())
}

fn is_supported(definition: &AwardDefinition) -> bool {
    definition.trigger == AwardTrigger::FixedAnnouncementDate
        && definition.evaluation_window == AwardEvaluationWindow::PreviousSeasonThroughAnnouncement
        && definition.recipient_kind == AwardRecipientKind::Player
        && matches!(
            definition.scope,
            AwardScopeKind::Global | AwardScopeKind::Competition
        )
}

fn announcement_date(
    calendar: &CalendarSystem,
    definition: &AwardDefinition,
    year: i64,
) -> ControllerResult<CalendarDate> {
    let month = definition.announcement_month_order_index.ok_or_else(|| {
        ControllerError::InvalidData(format!(
            "Award {} has no announcement month",
            definition.code
        ))
    })?;
    let day = definition.announcement_day_of_month.ok_or_else(|| {
        ControllerError::InvalidData(format!("Award {} has no announcement day", definition.code))
    })?;
    date_encoder::encode(
        calendar,
        &ResolvedCalendarDate::RegularDay {
            year,
            month_order_index: month,
            day_of_month: day,
            week_day_index: 0,
        },
    )
}

fn criteria_available(
    definition: &AwardDefinition,
    evidence: &[arlo_domain::AwardCandidateEvidence],
) -> bool {
    evidence.first().is_none_or(|candidate| {
        definition
            .criteria
            .iter()
            .chain(
                definition
                    .roster_slots
                    .iter()
                    .flat_map(|slot| slot.criteria.iter()),
            )
            .all(|criterion| {
                candidate
                    .metrics
                    .iter()
                    .any(|metric| metric.key == criterion.key)
            })
            && definition.tie_breaks.iter().all(|tie| {
                candidate
                    .metrics
                    .iter()
                    .any(|metric| metric.key == tie.metric_key)
            })
    })
}

async fn sources_ready(
    pool: &SqlitePool,
    definition: &AwardDefinition,
    sources: &[DatedSeasonSource],
) -> ControllerResult<bool> {
    let uses_titles = uses_metric(definition, "competition_titles_won");
    let uses_awards = uses_metric(definition, "individual_awards_won");
    for source in sources.iter().filter(|item| item.include_honors) {
        if uses_titles && sqlx::query_scalar::<_, i64>("SELECT 1 FROM titles t JOIN season_instances si ON si.competition_id = t.competition_id AND t.season_label = CAST(si.reference_year AS TEXT) WHERE si.id = ? LIMIT 1")
            .bind(source.source.season_id.to_string()).fetch_optional(pool).await?.is_none() {
            return Ok(false);
        }
        if uses_awards && sqlx::query_scalar::<_, i64>("SELECT 1 FROM award_season_jobs WHERE season_instance_id = ? AND status = 'Pending' LIMIT 1")
            .bind(source.source.season_id.to_string()).fetch_optional(pool).await?.is_some() {
            return Ok(false);
        }
    }
    Ok(true)
}

fn uses_metric(definition: &AwardDefinition, key: &str) -> bool {
    definition
        .criteria
        .iter()
        .chain(
            definition
                .roster_slots
                .iter()
                .flat_map(|slot| slot.criteria.iter()),
        )
        .any(|criterion| criterion.key == key)
}

async fn set_status(
    pool: &SqlitePool,
    job: &AnnouncementJob,
    status: &str,
    reason: &str,
) -> ControllerResult<()> {
    sqlx::query("UPDATE award_announcement_jobs SET status = ?, reason = ?, finished_at = CASE WHEN ? = 'Unavailable' THEN CURRENT_TIMESTAMP ELSE NULL END WHERE award_definition_id = ? AND announcement_year = ? AND scope_id = ?")
        .bind(status).bind(reason).bind(status).bind(&job.award_definition_id).bind(job.announcement_year).bind(&job.scope_id).execute(pool).await?;
    Ok(())
}
