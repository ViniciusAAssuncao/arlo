use crate::error::PersistenceResult;
use crate::models::MatchPlayerTouchesRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, Transaction};

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "player_id",
    "passes_attempted",
    "passes_received",
    "drives_recorded",
    "recoveries",
    "scoring_attempts",
    "total_touches",
    "turnovers_conceded",
];

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayerTouchesRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_player_touches (
            id,
            match_id,
            player_id,
            passes_attempted,
            passes_received,
            drives_recorded,
            recoveries,
            scoring_attempts,
            total_touches,
            turnovers_conceded
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.player_id)
    .bind(row.passes_attempted)
    .bind(row.passes_received)
    .bind(row.drives_recorded)
    .bind(row.recoveries)
    .bind(row.scoring_attempts)
    .bind(row.total_touches)
    .bind(row.turnovers_conceded)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayerTouchesRow],
) -> PersistenceResult<()> {
    execute_batch_insert(tx, "match_player_touches", COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(&row.player_id);
        b.push_bind(row.passes_attempted);
        b.push_bind(row.passes_received);
        b.push_bind(row.drives_recorded);
        b.push_bind(row.recoveries);
        b.push_bind(row.scoring_attempts);
        b.push_bind(row.total_touches);
        b.push_bind(row.turnovers_conceded);
    })
    .await
}
