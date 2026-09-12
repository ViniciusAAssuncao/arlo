use crate::error::PersistenceResult;
use crate::models::MatchPlayCallOutcomeRow;
use sqlx::{Sqlite, Transaction};

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
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}