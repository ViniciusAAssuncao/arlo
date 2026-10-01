use crate::error::PersistenceResult;
use crate::models::MatchPlayCallOutcomeRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const COLUMNS: &[&str] = &["id", "match_id", "play_call_id", "attempts", "successes"];

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPlayCallOutcomeRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_play_call_outcomes (
            id,
            match_id,
            play_call_id,
            attempts,
            successes
        ) VALUES (?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.play_call_id)
    .bind(row.attempts)
    .bind(row.successes)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchPlayCallOutcomeRow],
) -> PersistenceResult<()> {
    execute_batch_insert(tx, "match_play_call_outcomes", COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(&row.play_call_id);
        b.push_bind(row.attempts);
        b.push_bind(row.successes);
    })
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchPlayCallOutcomeRow>> {
    let rows = sqlx::query_as::<_, MatchPlayCallOutcomeRow>(
        r#"SELECT
            id,
            match_id,
            play_call_id,
            attempts,
            successes
        FROM match_play_call_outcomes
        WHERE match_id = ?"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
