use crate::error::PersistenceResult;
use crate::models::MatchPlayerAssistRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "player_id",
    "goalpoint_assists",
];

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerAssistRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_assists (
            id,
            match_id,
            player_id,
            goalpoint_assists
        ) VALUES (?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(row.goalpoint_assists)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerAssistRow],
) -> PersistenceResult<()> {
    execute_batch_insert(tx, "match_player_assists", COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(&row.player_id);
        b.push_bind(row.goalpoint_assists);
    })
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayerAssistRow>> {
    let rows = sqlx::query_as::<_, MatchPlayerAssistRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            goalpoint_assists
        FROM match_player_assists
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
) -> PersistenceResult<Option<MatchPlayerAssistRow>> {
    let row = sqlx::query_as::<_, MatchPlayerAssistRow>(
        r#"SELECT
            id,
            match_id,
            player_id,
            goalpoint_assists
        FROM match_player_assists
        WHERE match_id = ? AND player_id = ?"#,
    )
    .bind(match_id.to_string())
    .bind(player_id.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}
