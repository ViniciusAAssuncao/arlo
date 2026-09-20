use crate::error::{ControllerError, ControllerResult};
use arlo_domain::CollectiveAgreement;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> ControllerResult<Option<CollectiveAgreement>> {
    arlo_db::repositories::collective_agreement::get_by_id(pool, id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))
}

pub async fn list_all(pool: &SqlitePool) -> ControllerResult<Vec<CollectiveAgreement>> {
    arlo_db::repositories::collective_agreement::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))
}

pub async fn list_by_ids(
    pool: &SqlitePool,
    ids: &[Uuid],
) -> ControllerResult<Vec<CollectiveAgreement>> {
    arlo_db::repositories::collective_agreement::list_by_ids(pool, ids)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))
}