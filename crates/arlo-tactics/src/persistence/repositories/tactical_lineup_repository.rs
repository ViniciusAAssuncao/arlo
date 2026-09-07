use crate::error::{TacticsError, TacticsResult};
use crate::lineup::TacticalLineup;
use crate::persistence::models::rows::{TacticalLineupRow, TacticalLineupSlotRow};
use crate::persistence::models::slot_role_code::slot_role_to_code;
use arlo_db::repositories::fetch::{fetch_all_by_param, fetch_optional_by_param};
use sqlx::SqlitePool;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> TacticsResult<Option<TacticalLineup>> {
    let row = fetch_optional_by_param::<TacticalLineupRow>(
        pool,
        "SELECT id, team_id, formation_id, name, created_at_unix_seconds FROM tactical_lineups WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    let row = match row {
        Some(r) => r,
        None => return Ok(None),
    };

    let formation_id = Uuid::parse_str(&row.formation_id)?;
    let formation = arlo_db::repositories::formation::get_by_id(pool, formation_id)
        .await?
        .ok_or_else(|| {
            TacticsError::NotFound(format!(
                "Formation {} not found for lineup {}",
                formation_id, id
            ))
        })?;

    let slot_rows = sqlx::query_as::<_, TacticalLineupSlotRow>(
        "SELECT id, tactical_lineup_id, slot_index, player_id, slot_role FROM tactical_lineup_slots WHERE tactical_lineup_id = ? ORDER BY slot_index ASC",
    )
    .bind(id.to_string())
    .fetch_all(pool)
    .await?;

    let mut assignments = Vec::with_capacity(slot_rows.len());
    for sr in slot_rows {
        let idx = sr.slot_index as usize;
        let formation_slot = formation.slots().get(idx).ok_or_else(|| {
            TacticsError::InvalidLineup(format!(
                "Slot index {} out of bounds for formation {}",
                idx, formation_id
            ))
        })?;
        assignments.push(sr.to_domain(formation_slot.position())?);
    }

    Ok(Some(row.to_domain(assignments)?))
}

pub async fn list_by_team_id(
    pool: &SqlitePool,
    team_id: Uuid,
) -> TacticsResult<Vec<TacticalLineup>> {
    let rows = fetch_all_by_param::<TacticalLineupRow>(
        pool,
        "SELECT id, team_id, formation_id, name, created_at_unix_seconds FROM tactical_lineups WHERE team_id = ? ORDER BY created_at_unix_seconds ASC",
        &team_id.to_string(),
    )
    .await?;

    let mut lineups = Vec::with_capacity(rows.len());
    for row in rows {
        let formation_id = Uuid::parse_str(&row.formation_id)?;
        let formation = arlo_db::repositories::formation::get_by_id(pool, formation_id)
            .await?
            .ok_or_else(|| {
                TacticsError::NotFound(format!(
                    "Formation {} not found for lineup {}",
                    formation_id, row.id
                ))
            })?;

        let slot_rows = sqlx::query_as::<_, TacticalLineupSlotRow>(
            "SELECT id, tactical_lineup_id, slot_index, player_id, slot_role FROM tactical_lineup_slots WHERE tactical_lineup_id = ? ORDER BY slot_index ASC",
        )
        .bind(&row.id)
        .fetch_all(pool)
        .await?;

        let mut assignments = Vec::with_capacity(slot_rows.len());
        for sr in slot_rows {
            let idx = sr.slot_index as usize;
            let formation_slot = formation.slots().get(idx).ok_or_else(|| {
                TacticsError::InvalidLineup(format!(
                    "Slot index {} out of bounds for formation {}",
                    idx, formation_id
                ))
            })?;
            assignments.push(sr.to_domain(formation_slot.position())?);
        }

        lineups.push(row.to_domain(assignments)?);
    }

    Ok(lineups)
}

pub async fn insert(pool: &SqlitePool, lineup: &TacticalLineup) -> TacticsResult<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let mut tx = pool.begin().await?;

    sqlx::query(
        "INSERT INTO tactical_lineups (id, team_id, formation_id, name, created_at_unix_seconds) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(lineup.id().to_string())
    .bind(lineup.team_id().to_string())
    .bind(lineup.formation_id().to_string())
    .bind(lineup.name())
    .bind(timestamp)
    .execute(&mut *tx)
    .await?;

    for assignment in lineup.assignments() {
        let slot_id = Uuid::new_v4().to_string();
        let role_code = slot_role_to_code(assignment.slot_role());
        sqlx::query(
            "INSERT INTO tactical_lineup_slots (id, tactical_lineup_id, slot_index, player_id, slot_role) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(slot_id)
        .bind(lineup.id().to_string())
        .bind(assignment.formation_slot_index() as i32)
        .bind(assignment.player_id().to_string())
        .bind(role_code)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

pub async fn delete(pool: &SqlitePool, id: Uuid) -> TacticsResult<()> {
    sqlx::query("DELETE FROM tactical_lineups WHERE id = ?")
        .bind(id.to_string())
        .execute(pool)
        .await?;
    Ok(())
}