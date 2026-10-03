use crate::error::PersistenceResult;
use crate::models::season::{PowerRankingEntryRow, PowerRankingSnapshotRow};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_date(
    pool: &SqlitePool,
    season_instance_id: Uuid,
    year: i64,
    day_of_year: u32,
    model_version: u32,
) -> PersistenceResult<Option<PowerRankingSnapshotRow>> {
    let row = sqlx::query_as::<_, PowerRankingSnapshotRow>(
        "SELECT id, season_instance_id, year, day_of_year, model_version FROM power_ranking_snapshots WHERE season_instance_id = ? AND year = ? AND day_of_year = ? AND model_version = ?",
    )
    .bind(season_instance_id.to_string())
    .bind(year)
    .bind(day_of_year as i64)
    .bind(model_version as i64)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn get_latest(
    pool: &SqlitePool,
    season_instance_id: Uuid,
    model_version: u32,
) -> PersistenceResult<Option<PowerRankingSnapshotRow>> {
    let row = sqlx::query_as::<_, PowerRankingSnapshotRow>(
        "SELECT id, season_instance_id, year, day_of_year, model_version FROM power_ranking_snapshots WHERE season_instance_id = ? AND model_version = ? ORDER BY year DESC, day_of_year DESC LIMIT 1",
    )
    .bind(season_instance_id.to_string())
    .bind(model_version as i64)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn list_history(
    pool: &SqlitePool,
    season_instance_id: Uuid,
    model_version: u32,
) -> PersistenceResult<Vec<PowerRankingSnapshotRow>> {
    let rows = sqlx::query_as::<_, PowerRankingSnapshotRow>(
        "SELECT id, season_instance_id, year, day_of_year, model_version FROM power_ranking_snapshots WHERE season_instance_id = ? AND model_version = ? ORDER BY year, day_of_year",
    )
    .bind(season_instance_id.to_string())
    .bind(model_version as i64)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn list_entries(
    pool: &SqlitePool,
    snapshot_id: Uuid,
) -> PersistenceResult<Vec<PowerRankingEntryRow>> {
    let rows = sqlx::query_as::<_, PowerRankingEntryRow>(
        "SELECT snapshot_id, team_id, rank, rating, initial_rating, games_rated FROM power_ranking_entries WHERE snapshot_id = ? ORDER BY rank",
    )
    .bind(snapshot_id.to_string())
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
