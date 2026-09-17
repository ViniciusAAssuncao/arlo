use crate::error::PersistenceResult;
use crate::models::season::FixtureRow;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

pub async fn insert(tx: &mut Transaction<'_, Sqlite>, row: &FixtureRow) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO fixtures (
            id,
            season_stage_id,
            round_index,
            home_team_id,
            away_team_id,
            is_neutral_venue,
            venue_id,
            scheduled_year,
            scheduled_day_of_year,
            status,
            home_score,
            away_score,
            home_goal_points,
            away_goal_points
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.season_stage_id)
    .bind(row.round_index)
    .bind(&row.home_team_id)
    .bind(&row.away_team_id)
    .bind(row.is_neutral_venue)
    .bind(&row.venue_id)
    .bind(row.scheduled_year)
    .bind(row.scheduled_day_of_year)
    .bind(&row.status)
    .bind(row.home_score)
    .bind(row.away_score)
    .bind(row.home_goal_points)
    .bind(row.away_goal_points)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[FixtureRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}

pub async fn update(tx: &mut Transaction<'_, Sqlite>, row: &FixtureRow) -> PersistenceResult<()> {
    sqlx::query(
        r#"UPDATE fixtures SET
            round_index = ?,
            scheduled_year = ?,
            scheduled_day_of_year = ?,
            venue_id = ?,
            status = ?,
            home_score = ?,
            away_score = ?,
            home_goal_points = ?,
            away_goal_points = ?
        WHERE id = ?"#,
    )
    .bind(row.round_index)
    .bind(row.scheduled_year)
    .bind(row.scheduled_day_of_year)
    .bind(&row.venue_id)
    .bind(&row.status)
    .bind(row.home_score)
    .bind(row.away_score)
    .bind(row.home_goal_points)
    .bind(row.away_goal_points)
    .bind(&row.id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn update_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[FixtureRow],
) -> PersistenceResult<()> {
    for row in rows {
        update(tx, row).await?;
    }
    Ok(())
}

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> PersistenceResult<Option<FixtureRow>> {
    let row = sqlx::query_as::<_, FixtureRow>(
        "SELECT id, season_stage_id, round_index, home_team_id, away_team_id, is_neutral_venue, venue_id, scheduled_year, scheduled_day_of_year, status, home_score, away_score, home_goal_points, away_goal_points FROM fixtures WHERE id = ?",
    )
    .bind(id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn list_by_stage_id(
    pool: &SqlitePool,
    stage_id: Uuid,
) -> PersistenceResult<Vec<FixtureRow>> {
    let rows = sqlx::query_as::<_, FixtureRow>(
        "SELECT id, season_stage_id, round_index, home_team_id, away_team_id, is_neutral_venue, venue_id, scheduled_year, scheduled_day_of_year, status, home_score, away_score, home_goal_points, away_goal_points FROM fixtures WHERE season_stage_id = ? ORDER BY round_index, scheduled_year, scheduled_day_of_year ASC",
    )
    .bind(stage_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn list_scheduled_on_date(
    pool: &SqlitePool,
    scheduled_year: i64,
    scheduled_day_of_year: u32,
) -> PersistenceResult<Vec<FixtureRow>> {
    let rows = sqlx::query_as::<_, FixtureRow>(
        "SELECT id, season_stage_id, round_index, home_team_id, away_team_id, is_neutral_venue, venue_id, scheduled_year, scheduled_day_of_year, status, home_score, away_score, home_goal_points, away_goal_points FROM fixtures WHERE scheduled_year = ? AND scheduled_day_of_year = ? AND status = 'Scheduled' ORDER BY round_index ASC",
    )
    .bind(scheduled_year)
    .bind(scheduled_day_of_year as i32)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}