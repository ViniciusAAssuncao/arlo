use super::season_player_evidence::load_season_player_evidence;
use crate::error::ControllerResult;
use arlo_awards::{resolve_award, AwardError};
use arlo_domain::{
    AwardCandidateEvidence, AwardDefinition, AwardEvaluationWindow, AwardInstanceContext,
    AwardOrganizerPolicy, AwardRecipientKind, AwardTrigger,
};
use arlo_persistence::repositories::award_repository;
use sqlx::{FromRow, SqlitePool};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(FromRow)]
struct SeasonAwardJob {
    award_definition_id: String,
    season_instance_id: String,
    competition_id: String,
}

pub(crate) async fn process_season_award_jobs(pool: &SqlitePool) -> ControllerResult<u32> {
    let jobs = sqlx::query_as::<_, SeasonAwardJob>(
        "SELECT j.award_definition_id, j.season_instance_id, si.competition_id FROM award_season_jobs j JOIN season_instances si ON si.id = j.season_instance_id WHERE j.status = 'Pending' ORDER BY j.season_instance_id, j.award_definition_id"
    ).fetch_all(pool).await?;
    if jobs.is_empty() {
        return Ok(0);
    }
    let definitions = arlo_catalog::list_active_awards(pool).await?;
    let by_id: HashMap<Uuid, AwardDefinition> = definitions
        .into_iter()
        .map(|item| (item.id, item))
        .collect();
    let mut evidence_cache = HashMap::<Uuid, Vec<AwardCandidateEvidence>>::new();
    let mut resolved_count = 0;
    for job in jobs {
        let definition_id = Uuid::parse_str(&job.award_definition_id)?;
        let season_id = Uuid::parse_str(&job.season_instance_id)?;
        let competition_id = Uuid::parse_str(&job.competition_id)?;
        let Some(definition) = by_id.get(&definition_id) else {
            set_pending_reason(
                pool,
                definition_id,
                season_id,
                "Award definition is inactive or missing",
            )
            .await?;
            continue;
        };
        if definition.recipient_kind != AwardRecipientKind::Player
            || definition.trigger != AwardTrigger::SeasonCompleted
            || definition.evaluation_window != AwardEvaluationWindow::EntireSeason
        {
            set_pending_reason(
                pool,
                definition_id,
                season_id,
                "Season evidence builder does not support this definition",
            )
            .await?;
            continue;
        }
        if let std::collections::hash_map::Entry::Vacant(entry) = evidence_cache.entry(season_id) {
            entry.insert(load_season_player_evidence(pool, season_id, competition_id).await?);
        }
        let evidence = &evidence_cache[&season_id];
        let supported = evidence.first().is_none_or(|candidate| {
            definition.criteria.iter().all(|criterion| {
                candidate
                    .metrics
                    .iter()
                    .any(|metric| metric.key == criterion.key)
            }) && definition.tie_breaks.iter().all(|tie| {
                candidate
                    .metrics
                    .iter()
                    .any(|metric| metric.key == tie.metric_key)
            })
        });
        if !supported {
            set_pending_reason(
                pool,
                definition_id,
                season_id,
                "Award criteria require unavailable season metrics",
            )
            .await?;
            continue;
        }
        let context = AwardInstanceContext {
            period_key: format!("season:{season_id}"),
            scope_id: Some(competition_id),
            selection_model_version: 1,
        };
        let seed = award_seed(season_id, definition_id);
        let resolution = match resolve_award(definition, &context, evidence, seed) {
            Ok(value) => value,
            Err(AwardError::NoEligibleCandidates) => {
                set_unavailable(pool, definition_id, season_id, "No eligible candidates").await?;
                continue;
            }
            Err(error) => {
                set_pending_reason(pool, definition_id, season_id, &error.to_string()).await?;
                continue;
            }
        };
        let organizer_league_id =
            if definition.organizer_policy == AwardOrganizerPolicy::LeagueCommittee {
                let exists =
                    sqlx::query_scalar::<_, i64>("SELECT 1 FROM leagues WHERE competition_id = ?")
                        .bind(competition_id.to_string())
                        .fetch_optional(pool)
                        .await?
                        .is_some();
                exists.then_some(competition_id)
            } else {
                None
            };
        let mut tx = pool.begin().await?;
        award_repository::persist_resolution(&mut tx, &resolution, definition, organizer_league_id)
            .await?;
        sqlx::query(
            "UPDATE award_season_jobs SET status = 'Completed', reason = NULL, finished_at = CURRENT_TIMESTAMP WHERE award_definition_id = ? AND season_instance_id = ? AND status = 'Pending'"
        ).bind(definition_id.to_string()).bind(season_id.to_string()).execute(&mut *tx).await?;
        tx.commit().await?;
        resolved_count += 1;
    }
    Ok(resolved_count)
}

async fn set_pending_reason(
    pool: &SqlitePool,
    definition_id: Uuid,
    season_id: Uuid,
    reason: &str,
) -> ControllerResult<()> {
    sqlx::query("UPDATE award_season_jobs SET reason = ? WHERE award_definition_id = ? AND season_instance_id = ? AND status = 'Pending'")
        .bind(reason).bind(definition_id.to_string()).bind(season_id.to_string())
        .execute(pool).await?;
    Ok(())
}

async fn set_unavailable(
    pool: &SqlitePool,
    definition_id: Uuid,
    season_id: Uuid,
    reason: &str,
) -> ControllerResult<()> {
    sqlx::query("UPDATE award_season_jobs SET status = 'Unavailable', reason = ?, finished_at = CURRENT_TIMESTAMP WHERE award_definition_id = ? AND season_instance_id = ? AND status = 'Pending'")
        .bind(reason).bind(definition_id.to_string()).bind(season_id.to_string())
        .execute(pool).await?;
    Ok(())
}

fn award_seed(season_id: Uuid, definition_id: Uuid) -> u64 {
    season_id
        .as_bytes()
        .iter()
        .chain(definition_id.as_bytes())
        .fold(0xcbf29ce484222325_u64, |seed, byte| {
            seed.wrapping_mul(0x100000001b3) ^ u64::from(*byte)
        })
}
