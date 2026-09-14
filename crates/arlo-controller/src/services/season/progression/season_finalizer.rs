use crate::domain::season::PromotionRelegationOutcome;
use crate::error::ControllerResult;
use crate::services::season::persistence::finalize_season_instance;
use crate::services::season::progression::promotion_relegation_resolver::resolve_promotion_relegation;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn finalize_season(
    pool: &SqlitePool,
    competition_id: Uuid,
    season_instance_id: Uuid,
) -> ControllerResult<PromotionRelegationOutcome> {
    let outcome = resolve_promotion_relegation(pool, competition_id, season_instance_id).await?;
    finalize_season_instance(pool, season_instance_id).await?;
    Ok(outcome)
}
