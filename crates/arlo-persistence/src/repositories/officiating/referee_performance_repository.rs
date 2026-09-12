use crate::error::PersistenceResult;
use crate::models::MatchRefereePerformanceRow;
use sqlx::{Sqlite, Transaction};

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchRefereePerformanceRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_referee_performance (
            id,
            match_id,
            referee_id,
            role,
            calls_made,
            calls_correct,
            calls_incorrect,
            peace_referee_interventions
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.referee_id)
    .bind(&row.role)
    .bind(row.calls_made)
    .bind(row.calls_correct)
    .bind(row.calls_incorrect)
    .bind(row.peace_referee_interventions)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchRefereePerformanceRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}
