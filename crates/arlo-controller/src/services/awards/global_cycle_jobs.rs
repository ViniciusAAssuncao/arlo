use super::global_cycle_sources::ready_sources;
use super::global_player_evidence::load_global_player_evidence;
use crate::domain::calendar::{CalendarDate, CalendarSystem};
use crate::error::ControllerResult;
use crate::services::calendar::date_advancer;
use arlo_awards::{resolve_award, resolve_roster_award, AwardError};
use arlo_domain::{
    AwardDefinition, AwardEvaluationWindow, AwardInstanceContext, AwardRecipientKind,
    AwardResultKind, AwardScopeKind, AwardTrigger,
};
use arlo_persistence::repositories::{award_repository, award_roster_repository};
use sqlx::{FromRow, SqlitePool};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(FromRow)]
struct GlobalCycleJob {
    award_definition_id: String,
    reference_year: i64,
}

pub(crate) async fn process_global_cycle_jobs(
    pool: &SqlitePool,
    calendar: &CalendarSystem,
    date: CalendarDate,
) -> ControllerResult<u32> {
    enqueue_completed_cycles(pool).await?;
    let jobs = sqlx::query_as::<_, GlobalCycleJob>(
        "SELECT award_definition_id, reference_year FROM award_global_cycle_jobs WHERE status = 'Pending' ORDER BY reference_year, award_definition_id",
    )
    .fetch_all(pool)
    .await?;
    if jobs.is_empty() {
        return Ok(0);
    }
    let definitions: HashMap<_, _> = arlo_catalog::list_active_awards(pool)
        .await?
        .into_iter()
        .map(|definition| (definition.id, definition))
        .collect();
    let mut completed = 0;
    for job in jobs {
        let definition_id = Uuid::parse_str(&job.award_definition_id)?;
        let Some(definition) = definitions.get(&definition_id) else {
            set_reason(
                pool,
                definition_id,
                job.reference_year,
                "Award definition is inactive or missing",
                false,
            )
            .await?;
            continue;
        };
        if definition.scope != AwardScopeKind::Global
            || definition.trigger != AwardTrigger::SeasonCompleted
            || definition.evaluation_window != AwardEvaluationWindow::PreviousSeasonCycle
            || definition.recipient_kind != AwardRecipientKind::Player
        {
            set_reason(
                pool,
                definition_id,
                job.reference_year,
                "Global season evidence does not support this definition",
                false,
            )
            .await?;
            continue;
        }
        let Some(sources) = ready_sources(pool, definition, job.reference_year).await? else {
            set_reason(
                pool,
                definition_id,
                job.reference_year,
                "Waiting for eligible seasons to finish",
                false,
            )
            .await?;
            continue;
        };
        let ready = sqlx::query_as::<_, (Option<i64>, Option<i64>)>("SELECT ready_year, ready_day_of_year FROM award_global_cycle_jobs WHERE award_definition_id = ? AND reference_year = ?")
            .bind(definition_id.to_string()).bind(job.reference_year).fetch_one(pool).await?;
        let ready_date = if let (Some(year), Some(day)) = ready {
            CalendarDate::new(year, day as u32)
        } else {
            sqlx::query("UPDATE award_global_cycle_jobs SET ready_year = ?, ready_day_of_year = ?, reason = NULL WHERE award_definition_id = ? AND reference_year = ? AND ready_year IS NULL")
                .bind(date.year()).bind(i64::from(date.day_of_year())).bind(definition_id.to_string()).bind(job.reference_year).execute(pool).await?;
            date
        };
        if date
            < date_advancer::advance(
                calendar,
                &ready_date,
                i64::from(definition.announcement_delay_days),
            )
        {
            continue;
        }
        let evidence = load_global_player_evidence(pool, &sources).await?;
        if !criteria_available(definition, &evidence) {
            set_reason(
                pool,
                definition_id,
                job.reference_year,
                "Award criteria require unavailable global metrics",
                false,
            )
            .await?;
            continue;
        }
        let context = AwardInstanceContext {
            period_key: format!("season-cycle:{}", job.reference_year),
            scope_id: None,
            selection_model_version: 1,
        };
        let seed = cycle_seed(job.reference_year, definition_id);
        let mut tx = pool.begin().await?;
        let persisted = match definition.result_kind {
            AwardResultKind::SingleWinner => {
                match resolve_award(definition, &context, &evidence, seed) {
                    Ok(resolution) => {
                        award_repository::persist_resolution(&mut tx, &resolution, definition, None)
                            .await
                    }
                    Err(error) => {
                        drop(tx);
                        handle_resolution_error(pool, definition_id, job.reference_year, error)
                            .await?;
                        continue;
                    }
                }
            }
            AwardResultKind::Roster => {
                match resolve_roster_award(definition, &context, &evidence, seed) {
                    Ok(resolution) => {
                        award_roster_repository::persist_roster_resolution(
                            &mut tx,
                            &resolution,
                            definition,
                            None,
                        )
                        .await
                    }
                    Err(error) => {
                        drop(tx);
                        handle_resolution_error(pool, definition_id, job.reference_year, error)
                            .await?;
                        continue;
                    }
                }
            }
        }?;
        for source in &sources {
            sqlx::query("INSERT OR IGNORE INTO award_global_cycle_sources (award_instance_id, season_instance_id, competition_id) VALUES (?, ?, ?)")
                .bind(persisted.to_string())
                .bind(source.season_id.to_string())
                .bind(source.competition_id.to_string())
                .execute(&mut *tx)
                .await?;
        }
        sqlx::query("UPDATE award_global_cycle_jobs SET status = 'Completed', reason = NULL, finished_at = CURRENT_TIMESTAMP WHERE award_definition_id = ? AND reference_year = ? AND status = 'Pending'")
            .bind(definition_id.to_string())
            .bind(job.reference_year)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        completed += 1;
    }
    Ok(completed)
}

async fn enqueue_completed_cycles(pool: &SqlitePool) -> ControllerResult<()> {
    sqlx::query(
        "INSERT INTO award_global_cycle_jobs (award_definition_id, reference_year) SELECT DISTINCT d.id, si.reference_year FROM award_definitions d JOIN season_instances si ON si.status = 'Completed' JOIN competitions c ON c.id = si.competition_id WHERE d.active = 1 AND d.trigger_policy = '\"SeasonCompleted\"' AND d.evaluation_window = '\"PreviousSeasonCycle\"' AND d.scope_kind = 'Global' AND c.prestige >= COALESCE(d.minimum_competition_prestige, 0) AND (NOT EXISTS (SELECT 1 FROM award_eligibility_competitions ec WHERE ec.award_definition_id = d.id) OR EXISTS (SELECT 1 FROM award_eligibility_competitions ec WHERE ec.award_definition_id = d.id AND ec.competition_id = si.competition_id)) ON CONFLICT DO NOTHING",
    )
    .execute(pool)
    .await?;
    Ok(())
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
            .chain(
                definition
                    .dynamic_position_profiles
                    .iter()
                    .flat_map(|profile| profile.criteria.iter()),
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

async fn handle_resolution_error(
    pool: &SqlitePool,
    definition_id: Uuid,
    year: i64,
    error: AwardError,
) -> ControllerResult<()> {
    let unavailable = matches!(error, AwardError::NoEligibleCandidates);
    set_reason(pool, definition_id, year, &error.to_string(), unavailable).await
}

async fn set_reason(
    pool: &SqlitePool,
    definition_id: Uuid,
    year: i64,
    reason: &str,
    unavailable: bool,
) -> ControllerResult<()> {
    let status = if unavailable {
        "Unavailable"
    } else {
        "Pending"
    };
    sqlx::query("UPDATE award_global_cycle_jobs SET status = ?, reason = ?, finished_at = CASE WHEN ? = 'Unavailable' THEN CURRENT_TIMESTAMP ELSE NULL END WHERE award_definition_id = ? AND reference_year = ? AND status = 'Pending'")
        .bind(status)
        .bind(reason)
        .bind(status)
        .bind(definition_id.to_string())
        .bind(year)
        .execute(pool)
        .await?;
    Ok(())
}

fn cycle_seed(year: i64, definition_id: Uuid) -> u64 {
    year.to_le_bytes()
        .iter()
        .chain(definition_id.as_bytes())
        .fold(0xcbf29ce484222325_u64, |seed, byte| {
            seed.wrapping_mul(0x100000001b3) ^ u64::from(*byte)
        })
}
