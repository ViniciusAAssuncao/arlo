use crate::error::PersistenceResult;
use crate::models::MatchImpulseCriticalEventRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "sequence_number",
    "period",
    "seconds_in_period",
    "player_id",
    "value",
    "duration_seconds",
];

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
    execute_batch_insert(
        tx,
        "match_impulse_critical_events",
        COLUMNS,
        rows,
        |b, row| {
            b.push_bind(&row.id);
            b.push_bind(&row.match_id);
            b.push_bind(row.sequence_number);
            b.push_bind(row.period);
            b.push_bind(row.seconds_in_period);
            b.push_bind(&row.player_id);
            b.push_bind(row.value);
            b.push_bind(row.duration_seconds);
        },
    )
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchImpulseCriticalEventRow>> {
    let rows = sqlx::query_as::<_, MatchImpulseCriticalEventRow>(
        r#"SELECT
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            player_id,
            value,
            duration_seconds
        FROM match_impulse_critical_events
        WHERE match_id = ?
        ORDER BY sequence_number ASC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}