use super::validation::{ensure_existing_seeds, validate_publication};
use crate::error::PersistenceResult;
use arlo_analytics::{PowerRankingSnapshot, TeamPowerSeed};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn publish(
    pool: &SqlitePool,
    season_instance_id: Uuid,
    year: i64,
    day_of_year: u32,
    snapshot: &PowerRankingSnapshot,
    seeds: &[TeamPowerSeed],
) -> PersistenceResult<Uuid> {
    let initial_ratings = validate_publication(snapshot, seeds)?;
    let candidate_id = Uuid::new_v4();
    let mut tx = pool.begin().await?;
    sqlx::query(
        r#"INSERT INTO power_ranking_snapshots (
            id, season_instance_id, year, day_of_year, model_version
        ) VALUES (?, ?, ?, ?, ?)
        ON CONFLICT(season_instance_id, year, day_of_year, model_version) DO NOTHING"#,
    )
    .bind(candidate_id.to_string())
    .bind(season_instance_id.to_string())
    .bind(year)
    .bind(day_of_year as i64)
    .bind(snapshot.model_version() as i64)
    .execute(&mut *tx)
    .await?;

    ensure_existing_seeds(
        &mut tx,
        season_instance_id,
        snapshot.model_version(),
        &initial_ratings,
    )
    .await?;

    let snapshot_id: String = sqlx::query_scalar(
        "SELECT id FROM power_ranking_snapshots WHERE season_instance_id = ? AND year = ? AND day_of_year = ? AND model_version = ?",
    )
    .bind(season_instance_id.to_string())
    .bind(year)
    .bind(day_of_year as i64)
    .bind(snapshot.model_version() as i64)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query("DELETE FROM power_ranking_entries WHERE snapshot_id = ?")
        .bind(&snapshot_id)
        .execute(&mut *tx)
        .await?;

    for entry in snapshot.entries() {
        sqlx::query(
            r#"INSERT INTO power_ranking_entries (
                snapshot_id, team_id, rank, rating, initial_rating, games_rated
            ) VALUES (?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&snapshot_id)
        .bind(entry.team_id().to_string())
        .bind(entry.rank() as i64)
        .bind(entry.rating().value())
        .bind(initial_ratings[&entry.team_id()])
        .bind(entry.games_rated() as i64)
        .execute(&mut *tx)
        .await?;
    }

    let published_id = Uuid::parse_str(&snapshot_id)?;
    tx.commit().await?;
    Ok(published_id)
}
