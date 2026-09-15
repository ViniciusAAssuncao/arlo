use crate::domain::season::PromotionRelegationOutcome;
use crate::error::ControllerResult;
use crate::services::season::persistence::finalize_season_instance;
use crate::services::season::progression::champion_resolver::resolve_season_champion;
use crate::services::season::progression::league_movement_applier::resolve_and_apply_promotion_relegation;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn finalize_season(
    pool: &SqlitePool,
    competition_id: Uuid,
    season_instance_id: Uuid,
    knockout_champion: Option<Uuid>,
) -> ControllerResult<PromotionRelegationOutcome> {
    let outcome =
        resolve_and_apply_promotion_relegation(pool, competition_id, season_instance_id).await?;
    finalize_season_instance(pool, season_instance_id).await?;
    let title =
        resolve_season_champion(pool, competition_id, season_instance_id, knockout_champion).await?;
    arlo_db::repositories::title::insert(pool, &title)
        .await
        .map_err(|e| crate::error::ControllerError::InvalidData(e.to_string()))?;
    Ok(outcome)
}