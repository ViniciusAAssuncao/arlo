use crate::error::{ControllerError, ControllerResult};
use arlo_awards::{resolve_award, resolve_roster_award};
use arlo_domain::{AwardCandidateEvidence, AwardInstanceContext, AwardResultKind};
use arlo_persistence::repositories::{award_repository, award_roster_repository};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn resolve_and_persist_award(
    pool: &SqlitePool,
    award_code: &str,
    context: &AwardInstanceContext,
    candidates: &[AwardCandidateEvidence],
    seed: u64,
    organizer_league_id: Option<Uuid>,
) -> ControllerResult<Uuid> {
    let definition = arlo_catalog::get_award_by_code(pool, award_code)
        .await?
        .ok_or_else(|| ControllerError::NotFound(format!("Award {award_code} not found")))?;
    let mut tx = pool.begin().await?;
    let instance_id = match definition.result_kind {
        AwardResultKind::SingleWinner => {
            let resolution = resolve_award(&definition, context, candidates, seed)?;
            award_repository::persist_resolution(
                &mut tx,
                &resolution,
                &definition,
                organizer_league_id,
            )
            .await?
        }
        AwardResultKind::Roster => {
            let resolution = resolve_roster_award(&definition, context, candidates, seed)?;
            award_roster_repository::persist_roster_resolution(
                &mut tx,
                &resolution,
                &definition,
                organizer_league_id,
            )
            .await?
        }
    };
    tx.commit().await?;
    Ok(instance_id)
}
