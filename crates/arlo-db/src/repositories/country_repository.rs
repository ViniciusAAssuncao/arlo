use crate::error::DbResult;
use crate::models::CountryRow;
use crate::repositories::fetch::{fetch_all, fetch_all_by_param, fetch_optional_by_param};
use arlo_domain::Country;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<Country>> {
    let row = fetch_optional_by_param::<CountryRow>(
        pool,
        "SELECT id, name, continent_id, federation_id FROM countries WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<Country>> {
    let rows = fetch_all::<CountryRow>(
        pool,
        "SELECT id, name, continent_id, federation_id FROM countries",
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn list_by_continent_id(pool: &SqlitePool, continent_id: Uuid) -> DbResult<Vec<Country>> {
    let rows = fetch_all_by_param::<CountryRow>(
        pool,
        "SELECT id, name, continent_id, federation_id FROM countries WHERE continent_id = ?",
        &continent_id.to_string(),
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn list_by_federation_id(
    pool: &SqlitePool,
    federation_id: Uuid,
) -> DbResult<Vec<Country>> {
    let rows = fetch_all_by_param::<CountryRow>(
        pool,
        "SELECT id, name, continent_id, federation_id FROM countries WHERE federation_id = ?",
        &federation_id.to_string(),
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}
