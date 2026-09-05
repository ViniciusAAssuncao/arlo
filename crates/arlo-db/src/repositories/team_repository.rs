use crate::error::DbResult;
use crate::models::TeamRow;
use crate::repositories::fetch::{fetch_all, fetch_all_by_param, fetch_optional_by_param};
use arlo_domain::Team;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<Team>> {
    let row = fetch_optional_by_param::<TeamRow>(
        pool,
        "SELECT id, name, country_id, league_id, founded_at_unix_seconds, prestige, primary_color_hex, secondary_color_hex, home_venue_id FROM teams WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<Team>> {
    let rows = fetch_all::<TeamRow>(
        pool,
        "SELECT id, name, country_id, league_id, founded_at_unix_seconds, prestige, primary_color_hex, secondary_color_hex, home_venue_id FROM teams",
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn list_by_country_id(pool: &SqlitePool, country_id: Uuid) -> DbResult<Vec<Team>> {
    let rows = fetch_all_by_param::<TeamRow>(
        pool,
        "SELECT id, name, country_id, league_id, founded_at_unix_seconds, prestige, primary_color_hex, secondary_color_hex, home_venue_id FROM teams WHERE country_id = ?",
        &country_id.to_string(),
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn list_by_league_id(pool: &SqlitePool, league_id: Uuid) -> DbResult<Vec<Team>> {
    let rows = fetch_all_by_param::<TeamRow>(
        pool,
        "SELECT id, name, country_id, league_id, founded_at_unix_seconds, prestige, primary_color_hex, secondary_color_hex, home_venue_id FROM teams WHERE league_id = ?",
        &league_id.to_string(),
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}
