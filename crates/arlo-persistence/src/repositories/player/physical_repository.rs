use crate::error::PersistenceResult;
use crate::models::MatchPlayerPhysicalRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "player_id",
    "end_energy_level",
    "peak_anaerobic_depletion",
    "total_distance_covered",
    "intra_match_recovery_amount",
];

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
    execute_batch_insert(tx, "match_player_physical", COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(&row.player_id);
        b.push_bind(row.end_energy_level);
        b.push_bind(row.peak_anaerobic_depletion);
        b.push_bind(row.total_distance_covered);
        b.push_bind(row.intra_match_recovery_amount);
    })
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerPhysicalRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerPhysicalRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            end_energy_level,
            peak_anaerobic_depletion,
            total_distance_covered,
            intra_match_recovery_amount
        FROM match_player_physical
        WHERE match_id = ?"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_by_match_id_and_player_id(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> PersistenceResult<Option<MatchPlayerPhysicalRow>> {
    let row = sqlx::query_as::<_, MatchPlayerPhysicalRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            end_energy_level,
            peak_anaerobic_depletion,
            total_distance_covered,
            intra_match_recovery_amount
        FROM match_player_physical
        WHERE match_id = ? AND player_id = ?"#,
    )
    .bind(match_id.to_string())
    .bind(player_id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
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

pub async fn list_latest_by_player_ids(
    pool: &SqlitePool,
    player_ids: &[Uuid],
) -> PersistenceResult<Vec<MatchPlayerPhysicalRow>> {
    if player_ids.is_empty() {
        return Ok(Vec::new());
    }

    let placeholders = std::iter::repeat("?").take(player_ids.len()).collect::<Vec<_>>().join(", ");

    let sql = format!(
        r#"SELECT
            id,
            match_id,
            player_id,
            end_energy_level,
            peak_anaerobic_depletion,
            total_distance_covered,
            intra_match_recovery_amount
        FROM (
            SELECT
                p.id,
                p.match_id,
                p.player_id,
                p.end_energy_level,
                p.peak_anaerobic_depletion,
                p.total_distance_covered,
                p.intra_match_recovery_amount,
                ROW_NUMBER() OVER (
                    PARTITION BY p.player_id
                    ORDER BY COALESCE(f.scheduled_year, 0) DESC, COALESCE(f.scheduled_day_of_year, 0) DESC, COALESCE(m.completed_at_unix_seconds, 0) DESC, p.rowid DESC
                ) AS rn
            FROM match_player_physical p
            LEFT JOIN matches m ON p.match_id = m.id
            LEFT JOIN fixtures f ON m.fixture_id = f.id
            WHERE p.player_id IN ({})
        )
        WHERE rn = 1"#,
        placeholders
    );

    let mut query = sqlx::query_as::<_, MatchPlayerPhysicalRow>(&sql);
    for id in player_ids {
        query = query.bind(id.to_string());
    }

    let rows = query.fetch_all(pool).await?;
    Ok(rows)
}
