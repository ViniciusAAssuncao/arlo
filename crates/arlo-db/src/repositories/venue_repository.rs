use crate::error::DbResult;
use crate::models::VenueRow;
use crate::repositories::fetch::{fetch_all, fetch_all_by_param, fetch_optional_by_param};
use arlo_domain::Venue;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<Venue>> {
    let row = fetch_optional_by_param::<VenueRow>(
        pool,
        "SELECT id, name, kind, owner_team_id, country_id, capacity, pitch_length_mirim, pitch_width_mirim FROM venues WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<Venue>> {
    let rows = fetch_all::<VenueRow>(
        pool,
        "SELECT id, name, kind, owner_team_id, country_id, capacity, pitch_length_mirim, pitch_width_mirim FROM venues",
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn list_by_owner_team_id(pool: &SqlitePool, owner_team_id: Uuid) -> DbResult<Vec<Venue>> {
    let rows = fetch_all_by_param::<VenueRow>(
        pool,
        "SELECT id, name, kind, owner_team_id, country_id, capacity, pitch_length_mirim, pitch_width_mirim FROM venues WHERE owner_team_id = ?",
        &owner_team_id.to_string(),
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn list_by_country_id(pool: &SqlitePool, country_id: Uuid) -> DbResult<Vec<Venue>> {
    let rows = fetch_all_by_param::<VenueRow>(
        pool,
        "SELECT id, name, kind, owner_team_id, country_id, capacity, pitch_length_mirim, pitch_width_mirim FROM venues WHERE country_id = ?",
        &country_id.to_string(),
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}
