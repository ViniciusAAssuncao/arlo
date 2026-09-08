use crate::error::DbResult;
use crate::models::CompetitionRow;
use crate::repositories::fetch::{fetch_all, fetch_all_by_param, fetch_optional_by_param};
use arlo_domain::Competition;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<Competition>> {
    let row = fetch_optional_by_param::<CompetitionRow>(
        pool,
        "SELECT id, name, federation_id, country_id, scope, kind, prestige FROM competitions WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<Competition>> {
    let rows = fetch_all::<CompetitionRow>(
        pool,
        "SELECT id, name, federation_id, country_id, scope, kind, prestige FROM competitions",
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
) -> DbResult<Vec<Competition>> {
    let rows = fetch_all_by_param::<CompetitionRow>(
        pool,
        "SELECT id, name, federation_id, country_id, scope, kind, prestige FROM competitions WHERE federation_id = ?",
        &federation_id.to_string(),
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn list_by_country_id(pool: &SqlitePool, country_id: Uuid) -> DbResult<Vec<Competition>> {
    let rows = fetch_all_by_param::<CompetitionRow>(
        pool,
        "SELECT id, name, federation_id, country_id, scope, kind, prestige FROM competitions WHERE country_id = ?",
        &country_id.to_string(),
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}
