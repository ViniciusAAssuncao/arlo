use crate::error::PersistenceResult;
use crate::models::MatchPlayerDrivesRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "player_id",
    "total_drives",
    "central_drives",
    "left_lateral_drives",
    "right_lateral_drives",
    "lateral_drives",
    "max_drives_in_series",
];

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerDrivesRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_drives (
            id,
            match_id,
            player_id,
            total_drives,
            central_drives,
            left_lateral_drives,
            right_lateral_drives,
            lateral_drives,
            max_drives_in_series
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(row.total_drives)
    .bind(row.central_drives)
    .bind(row.left_lateral_drives)
    .bind(row.right_lateral_drives)
    .bind(row.lateral_drives)
    .bind(row.max_drives_in_series)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerDrivesRow],
) -> PersistenceResult<()> {
    execute_batch_insert(tx, "match_player_drives", COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(&row.player_id);
        b.push_bind(row.total_drives);
        b.push_bind(row.central_drives);
        b.push_bind(row.left_lateral_drives);
        b.push_bind(row.right_lateral_drives);
        b.push_bind(row.lateral_drives);
        b.push_bind(row.max_drives_in_series);
    })
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerDrivesRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerDrivesRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            total_drives,
            central_drives,
            left_lateral_drives,
            right_lateral_drives,
            lateral_drives,
            max_drives_in_series
        FROM match_player_drives
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
) -> PersistenceResult<Option<MatchPlayerDrivesRow>> {
    let row = sqlx::query_as::<_, MatchPlayerDrivesRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            total_drives,
            central_drives,
            left_lateral_drives,
            right_lateral_drives,
            lateral_drives,
            max_drives_in_series
        FROM match_player_drives
        WHERE match_id = ? AND player_id = ?"#,
    )
    .bind(match_id.to_string())
    .bind(player_id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}
