use crate::error::PersistenceResult;
use crate::models::MatchSubstitutionRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, Transaction};

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "sequence_number",
    "period",
    "seconds_in_period",
    "team_id",
    "player_out_id",
    "player_in_id",
    "reason",
];

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchSubstitutionRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_substitutions (
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            team_id,
            player_out_id,
            player_in_id,
            reason
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(row.sequence_number)
    .bind(row.period)
    .bind(row.seconds_in_period)
    .bind(&row.team_id)
    .bind(&row.player_out_id)
    .bind(&row.player_in_id)
    .bind(&row.reason)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchSubstitutionRow],
) -> PersistenceResult<()> {
    execute_batch_insert(tx, "match_substitutions", COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(row.sequence_number);
        b.push_bind(row.period);
        b.push_bind(row.seconds_in_period);
        b.push_bind(&row.team_id);
        b.push_bind(&row.player_out_id);
        b.push_bind(&row.player_in_id);
        b.push_bind(&row.reason);
    })
    .await
}
