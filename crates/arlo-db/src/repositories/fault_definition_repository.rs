use crate::error::DbResult;
use crate::models::FaultDefinitionRow;
use crate::repositories::fetch::{fetch_all, fetch_optional_by_param};
use arlo_domain::FaultDefinition;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<FaultDefinition>> {
    let row = fetch_optional_by_param::<FaultDefinitionRow>(
        pool,
        "SELECT id, code, description, severity FROM fault_definitions WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn get_by_code(pool: &SqlitePool, code: &str) -> DbResult<Option<FaultDefinition>> {
    let row = fetch_optional_by_param::<FaultDefinitionRow>(
        pool,
        "SELECT id, code, description, severity FROM fault_definitions WHERE code = ?",
        code,
    )
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<FaultDefinition>> {
    let rows = fetch_all::<FaultDefinitionRow>(
        pool,
        "SELECT id, code, description, severity FROM fault_definitions",
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}