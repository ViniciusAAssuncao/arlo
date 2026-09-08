use crate::error::DbResult;
use crate::models::AttributeDefinitionRow;
use crate::repositories::fetch::{fetch_all, fetch_all_by_param, fetch_optional_by_param};
use arlo_domain::{AttributeDefinition, AttributeTarget};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<AttributeDefinition>> {
    let row = fetch_optional_by_param::<AttributeDefinitionRow>(
        pool,
        "SELECT id, key, display_name, category, applies_to FROM attribute_definitions WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<AttributeDefinition>> {
    let rows = fetch_all::<AttributeDefinitionRow>(
        pool,
        "SELECT id, key, display_name, category, applies_to FROM attribute_definitions",
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn list_by_applies_to(
    pool: &SqlitePool,
    target: AttributeTarget,
) -> DbResult<Vec<AttributeDefinition>> {
    let target_str = match target {
        AttributeTarget::Player => "Player",
        AttributeTarget::Manager => "Manager",
    };
    let rows = fetch_all_by_param::<AttributeDefinitionRow>(
        pool,
        "SELECT id, key, display_name, category, applies_to FROM attribute_definitions WHERE applies_to = ?",
        target_str,
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}
