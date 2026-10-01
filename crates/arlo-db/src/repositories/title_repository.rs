use crate::error::DbResult;
use crate::models::TitleRow;
use crate::repositories::fetch::{fetch_all, fetch_all_by_param, fetch_optional_by_param};
use arlo_domain::Title;
use sqlx::SqlitePool;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<Title>> {
    let row = fetch_optional_by_param::<TitleRow>(
        pool,
        "SELECT id, competition_id, season_label, winner_team_id, winner_federation_id, created_at_unix_seconds FROM titles WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn get_latest_by_competition_id(
    pool: &SqlitePool,
    competition_id: Uuid,
) -> DbResult<Option<Title>> {
    let row = fetch_optional_by_param::<TitleRow>(
        pool,
        "SELECT id, competition_id, season_label, winner_team_id, winner_federation_id, created_at_unix_seconds FROM titles WHERE competition_id = ? ORDER BY created_at_unix_seconds DESC, rowid DESC LIMIT 1",
        &competition_id.to_string(),
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
        "SELECT id, competition_id, season_label, winner_team_id, winner_federation_id, created_at_unix_seconds FROM titles",
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
        "SELECT id, competition_id, season_label, winner_team_id, winner_federation_id, created_at_unix_seconds FROM titles WHERE competition_id = ?",
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
    list_by_winner_team_id(pool, team_id).await
}

pub async fn list_by_winner_team_id(pool: &SqlitePool, team_id: Uuid) -> DbResult<Vec<Title>> {
    let rows = fetch_all_by_param::<TitleRow>(
        pool,
        "SELECT id, competition_id, season_label, winner_team_id, winner_federation_id, created_at_unix_seconds FROM titles WHERE winner_team_id = ? ORDER BY created_at_unix_seconds DESC, rowid DESC",
        &team_id.to_string(),
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn list_by_federation_id(pool: &SqlitePool, federation_id: Uuid) -> DbResult<Vec<Title>> {
    let rows = fetch_all_by_param::<TitleRow>(
        pool,
        "SELECT id, competition_id, season_label, winner_team_id, winner_federation_id, created_at_unix_seconds FROM titles WHERE winner_federation_id = ? ORDER BY created_at_unix_seconds DESC, rowid DESC",
        &federation_id.to_string(),
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn insert(pool: &SqlitePool, title: &Title) -> DbResult<()> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let row = TitleRow::from_domain(title, now);
    sqlx::query(
        "INSERT INTO titles (id, competition_id, season_label, winner_team_id, winner_federation_id, created_at_unix_seconds) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&row.id)
    .bind(&row.competition_id)
    .bind(&row.season_label)
    .bind(&row.winner_team_id)
    .bind(&row.winner_federation_id)
    .bind(row.created_at_unix_seconds)
    .execute(pool)
    .await?;
    Ok(())
}
