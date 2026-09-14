use crate::error::ControllerResult;
use crate::services::season::persistence::finalize_season_instance;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn finalize_season(
    pool: &SqlitePool,
    _competition_id: Uuid,
    season_instance_id: Uuid,
) -> ControllerResult<()> {
    finalize_season_instance(pool, season_instance_id).await
}