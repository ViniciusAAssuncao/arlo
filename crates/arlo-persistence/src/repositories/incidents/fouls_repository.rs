use crate::error::PersistenceResult;
use crate::models::MatchFoulRow;
use sqlx::{Sqlite, Transaction};

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
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}
