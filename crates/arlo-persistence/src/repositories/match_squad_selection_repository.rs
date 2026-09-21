use crate::error::PersistenceResult;
use crate::models::MatchSquadSelectionRow;
use crate::repositories::batching::execute_batch_insert;
use sqlx::{Sqlite, Transaction};

const COLUMNS: &[&str] = &[
    "id",
    "match_id",
    "team_id",
    "player_id",
    "was_starter",
    "formation_slot_index",
    "slot_role",
    "was_used",
    "final_availability_status",
    "final_suspended_remaining_seconds",
];

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
    execute_batch_insert(tx, "match_squad_selections", COLUMNS, rows, |b, row| {
        b.push_bind(&row.id);
        b.push_bind(&row.match_id);
        b.push_bind(&row.team_id);
        b.push_bind(&row.player_id);
        b.push_bind(row.was_starter);
        b.push_bind(row.formation_slot_index);
        b.push_bind(&row.slot_role);
        b.push_bind(row.was_used);
        b.push_bind(&row.final_availability_status);
        b.push_bind(row.final_suspended_remaining_seconds);
    })
    .await
}
