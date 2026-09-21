use crate::error::PersistenceResult;
use crate::models::MatchPlayerAssistRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, Transaction};

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
