use crate::error::PersistenceResult;
use crate::models::MatchAvailabilityChangeRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, Transaction};

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "sequence_number",
    "period",
    "seconds_in_period",
    "player_id",
    "team_id",
    "previous_status",
    "new_status",
    "remaining_seconds",
];

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchAvailabilityChangeRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_availability_changes (
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            player_id,
            team_id,
            previous_status,
            new_status,
            remaining_seconds
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(row.sequence_number)
    .bind(row.period)
    .bind(row.seconds_in_period)
    .bind(&row.player_id)
    .bind(&row.team_id)
    .bind(&row.previous_status)
    .bind(&row.new_status)
    .bind(row.remaining_seconds)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchAvailabilityChangeRow],
) -> PersistenceResult<()> {
    execute_batch_insert(
        tx,
        "match_availability_changes",
        COLUMNS,
        rows,
        |b, row| {
            b.push_bind(&row.id);
            b.push_bind(&row.match_id);
            b.push_bind(row.sequence_number);
            b.push_bind(row.period);
            b.push_bind(row.seconds_in_period);
            b.push_bind(&row.player_id);
            b.push_bind(&row.team_id);
            b.push_bind(&row.previous_status);
            b.push_bind(&row.new_status);
            b.push_bind(row.remaining_seconds);
        },
    )
    .await
}
