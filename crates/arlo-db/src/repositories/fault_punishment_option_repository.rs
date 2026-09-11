use crate::error::DbResult;
use crate::models::FaultPunishmentOptionRow;
use crate::repositories::fetch::fetch_all_by_param;
use arlo_domain::FaultPunishmentOption;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn list_by_fault_definition_id(
    pool: &SqlitePool,
    fault_definition_id: Uuid,
) -> DbResult<Vec<FaultPunishmentOption>> {
    let rows = fetch_all_by_param::<FaultPunishmentOptionRow>(
        pool,
        "SELECT id, fault_definition_id, kind, magnitude_min, magnitude_max FROM fault_punishment_options WHERE fault_definition_id = ?",
        &fault_definition_id.to_string(),
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}