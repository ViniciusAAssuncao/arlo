use crate::error::PersistenceResult;
use crate::models::MatchImpulseCriticalEventRow;
use sqlx::{Sqlite, Transaction};

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchImpulseCriticalEventRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_impulse_critical_events (
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            player_id,
            value,
            duration_seconds
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(row.sequence_number)
    .bind(row.period)
    .bind(row.seconds_in_period)
    .bind(&row.player_id)
    .bind(row.value)
    .bind(row.duration_seconds)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchImpulseCriticalEventRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}
