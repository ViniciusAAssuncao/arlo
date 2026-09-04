use crate::error::DbResult;
use crate::models::TitleRow;
use crate::repositories::fetch::{fetch_all, fetch_all_by_param, fetch_optional_by_param};
use arlo_domain::Title;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<Title>> {
    let row = fetch_optional_by_param::<TitleRow>(
        pool,
        "SELECT id, competition_id, season_label, winner_team_id, winner_federation_id FROM titles WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<Title>> {
    let rows = fetch_all::<TitleRow>(
        pool,
        "SELECT id, competition_id, season_label, winner_team_id, winner_federation_id FROM titles",
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn list_by_competition_id(
    pool: &SqlitePool,
    competition_id: Uuid,
) -> DbResult<Vec<Title>> {
    let rows = fetch_all_by_param::<TitleRow>(
        pool,
        "SELECT id, competition_id, season_label, winner_team_id, winner_federation_id FROM titles WHERE competition_id = ?",
        &competition_id.to_string(),
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn list_by_team_id(pool: &SqlitePool, team_id: Uuid) -> DbResult<Vec<Title>> {
    let rows = fetch_all_by_param::<TitleRow>(
        pool,
        "SELECT id, competition_id, season_label, winner_team_id, winner_federation_id FROM titles WHERE winner_team_id = ?",
        &team_id.to_string(),
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
) -> DbResult<Vec<Title>> {
    let rows = fetch_all_by_param::<TitleRow>(
        pool,
        "SELECT id, competition_id, season_label, winner_team_id, winner_federation_id FROM titles WHERE winner_federation_id = ?",
        &federation_id.to_string(),
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}