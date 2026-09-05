use crate::error::DbResult;
use crate::models::ContinentRow;
use crate::repositories::fetch::{fetch_all, fetch_optional_by_param};
use arlo_domain::Continent;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<Continent>> {
    let row = fetch_optional_by_param::<ContinentRow>(
        pool,
        "SELECT id, name FROM continents WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<Continent>> {
    let rows = fetch_all::<ContinentRow>(pool, "SELECT id, name FROM continents").await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}