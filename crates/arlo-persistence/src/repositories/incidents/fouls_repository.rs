use crate::error::PersistenceResult;
use crate::models::MatchFoulRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "sequence_number",
    "period",
    "seconds_in_period",
    "offending_player_id",
    "offending_team_id",
    "opposing_player_id",
    "opposing_team_id",
    "origin",
    "original_call_correct",
    "peace_referee_intervened",
    "fault_definition_id",
    "punishment_kind",
    "punishment_magnitude",
];

pub async fn insert(tx: &mut Transaction<'_, Sqlite>, row: &MatchFoulRow) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_fouls (
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            offending_player_id,
            offending_team_id,
            opposing_player_id,
            opposing_team_id,
            origin,
            original_call_correct,
            peace_referee_intervened,
            fault_definition_id,
            punishment_kind,
            punishment_magnitude
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(row.sequence_number)
    .bind(row.period)
    .bind(row.seconds_in_period)
    .bind(&row.offending_player_id)
    .bind(&row.offending_team_id)
    .bind(&row.opposing_player_id)
    .bind(&row.opposing_team_id)
    .bind(&row.origin)
    .bind(row.original_call_correct)
    .bind(row.peace_referee_intervened)
    .bind(&row.fault_definition_id)
    .bind(&row.punishment_kind)
    .bind(row.punishment_magnitude)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchFoulRow],
) -> PersistenceResult<()> {
    execute_batch_insert(tx, "match_fouls", COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(row.sequence_number);
        b.push_bind(row.period);
        b.push_bind(row.seconds_in_period);
        b.push_bind(&row.offending_player_id);
        b.push_bind(&row.offending_team_id);
        b.push_bind(&row.opposing_player_id);
        b.push_bind(&row.opposing_team_id);
        b.push_bind(&row.origin);
        b.push_bind(row.original_call_correct);
        b.push_bind(row.peace_referee_intervened);
        b.push_bind(&row.fault_definition_id);
        b.push_bind(&row.punishment_kind);
        b.push_bind(row.punishment_magnitude);
    })
    .await
}

pub async fn list_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchFoulRow>> {
    let rows = sqlx::query_as::<_, MatchFoulRow>(
        r#"SELECT
            id,
            match_id,
            sequence_number,
            period,
            seconds_in_period,
            offending_player_id,
            offending_team_id,
            opposing_player_id,
            opposing_team_id,
            origin,
            original_call_correct,
            peace_referee_intervened,
            fault_definition_id,
            punishment_kind,
            punishment_magnitude
        FROM match_fouls
        WHERE match_id = ?
        ORDER BY sequence_number ASC"#,
    )
    .bind(match_id.to_string())
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
