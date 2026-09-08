use crate::error::DbResult;
use crate::models::FederationRow;
use crate::repositories::fetch::{fetch_all, fetch_all_by_param, fetch_optional_by_param};
use arlo_domain::Federation;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<Federation>> {
    let row = fetch_optional_by_param::<FederationRow>(
        pool,
        "SELECT id, name, scope, continent_id, parent_federation_id, prestige FROM federations WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<Federation>> {
    let rows = fetch_all::<FederationRow>(
        pool,
        "SELECT id, name, scope, continent_id, parent_federation_id, prestige FROM federations",
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn list_by_continent_id(
    pool: &SqlitePool,
    continent_id: Uuid,
) -> DbResult<Vec<Federation>> {
    let rows = fetch_all_by_param::<FederationRow>(
        pool,
        "SELECT id, name, scope, continent_id, parent_federation_id, prestige FROM federations WHERE continent_id = ?",
        &continent_id.to_string(),
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn list_by_parent_federation_id(
    pool: &SqlitePool,
    parent_federation_id: Uuid,
) -> DbResult<Vec<Federation>> {
    let rows = fetch_all_by_param::<FederationRow>(
        pool,
        "SELECT id, name, scope, continent_id, parent_federation_id, prestige FROM federations WHERE parent_federation_id = ?",
        &parent_federation_id.to_string(),
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}
