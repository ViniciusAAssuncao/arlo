use crate::error::PersistenceResult;
use crate::models::MatchPlayerPhysicalRow;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerPhysicalRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_physical (
            id,
            match_id,
            player_id,
            end_energy_level,
            peak_anaerobic_depletion,
            total_distance_covered,
            intra_match_recovery_amount
        ) VALUES (?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(row.end_energy_level)
    .bind(row.peak_anaerobic_depletion)
    .bind(row.total_distance_covered)
    .bind(row.intra_match_recovery_amount)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerPhysicalRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}

pub async fn get_latest_by_player_id(
    pool: &SqlitePool,
    player_id: Uuid,
) -> PersistenceResult<Option<MatchPlayerPhysicalRow>> {
    let row = sqlx::query_as::<_, MatchPlayerPhysicalRow>(
        r#"SELECT
            p.id,
            p.match_id,
            p.player_id,
            p.end_energy_level,
            p.peak_anaerobic_depletion,
            p.total_distance_covered,
            p.intra_match_recovery_amount
        FROM match_player_physical p
        LEFT JOIN matches m ON p.match_id = m.id
        LEFT JOIN fixtures f ON m.fixture_id = f.id
        WHERE p.player_id = ?
        ORDER BY COALESCE(f.scheduled_year, 0) DESC, COALESCE(f.scheduled_day_of_year, 0) DESC, COALESCE(m.completed_at_unix_seconds, 0) DESC, p.rowid DESC
        LIMIT 1"#,
    )
    .bind(player_id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}