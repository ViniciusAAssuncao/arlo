use crate::error::ControllerResult;
use arlo_persistence::models::season::SeasonInstanceRow;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn resolve_active_season(
    pool: &SqlitePool,
    competition_id: Uuid,
) -> ControllerResult<Option<SeasonInstanceRow>> {
    let season_instances =
        arlo_persistence::repositories::season::season_instances::list_by_competition_id(
            pool,
            competition_id,
        )
        .await?;

    Ok(season_instances
        .into_iter()
        .find(|s| s.status == "Active" || s.status == "Pending"))
}
