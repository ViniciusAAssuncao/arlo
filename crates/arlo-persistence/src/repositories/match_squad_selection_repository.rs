use crate::error::PersistenceResult;
use crate::models::MatchSquadSelectionRow;
use sqlx::{Sqlite, Transaction};

pub async fn insert(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchSquadSelectionRow,
) -> PersistenceResult<()> {
    sqlx::query(
        r#"INSERT INTO match_squad_selections (
            id,
            match_id,
            team_id,
            player_id,
            was_starter,
            formation_slot_index,
            slot_role,
            was_used,
            final_availability_status,
            final_suspended_remaining_seconds
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(&row.id)
    .bind(&row.match_id)
    .bind(&row.team_id)
    .bind(&row.player_id)
    .bind(row.was_starter)
    .bind(row.formation_slot_index)
    .bind(&row.slot_role)
    .bind(row.was_used)
    .bind(&row.final_availability_status)
    .bind(row.final_suspended_remaining_seconds)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_batch(
    tx: &mut Transaction<'_, Sqlite>,
    rows: &[MatchSquadSelectionRow],
) -> PersistenceResult<()> {
    for row in rows {
        insert(tx, row).await?;
    }
    Ok(())
}
