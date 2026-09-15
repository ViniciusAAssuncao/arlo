use crate::domain::season::PromotionRelegationOutcome;
use crate::error::ControllerResult;
use crate::services::season::progression::league_movement_applier;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn resolve_and_apply_promotion_relegation(
    pool: &SqlitePool,
    competition_id: Uuid,
    season_instance_id: Uuid,
) -> ControllerResult<PromotionRelegationOutcome> {
    league_movement_applier::resolve_and_apply_promotion_relegation(
        pool,
        competition_id,
        season_instance_id,
    )
    .await
}