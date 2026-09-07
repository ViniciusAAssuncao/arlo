use crate::error::{TacticsError, TacticsResult};
use crate::instructions::TeamInstructions;
use crate::persistence::models::instruction_key_code::{
    instruction_key_to_code, InstructionKey,
};
use crate::persistence::models::rows::{
    TacticalInstructionValueRow, TeamTacticalProfileRow,
};
use crate::persistence::models::tactical_phase::TacticalPhase;
use crate::persistence::models::tactical_phase_code::tactical_phase_to_code;
use arlo_db::repositories::fetch::{fetch_all_by_param, fetch_optional_by_param};
use sqlx::SqlitePool;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub async fn get_active_by_team_id(
    pool: &SqlitePool,
    team_id: Uuid,
) -> TacticsResult<Option<TeamInstructions>> {
    let profile_row = fetch_optional_by_param::<TeamTacticalProfileRow>(
        pool,
        "SELECT id, team_id, name, is_active, created_at_unix_seconds FROM team_tactical_profiles WHERE team_id = ? AND is_active = 1",
        &team_id.to_string(),
    )
    .await?;

    let profile_row = match profile_row {
        Some(r) => r,
        None => return Ok(None),
    };

    let value_rows = fetch_all_by_param::<TacticalInstructionValueRow>(
        pool,
        "SELECT id, team_tactical_profile_id, phase, instruction_key, value FROM tactical_instruction_values WHERE team_tactical_profile_id = ?",
        &profile_row.id,
    )
    .await?;

    let instructions = profile_row.to_domain(&value_rows)?;
    Ok(Some(instructions))
}

pub async fn list_profiles_by_team_id(
    pool: &SqlitePool,
    team_id: Uuid,
) -> TacticsResult<Vec<(TeamTacticalProfileRow, TeamInstructions)>> {
    let profile_rows = fetch_all_by_param::<TeamTacticalProfileRow>(
        pool,
        "SELECT id, team_id, name, is_active, created_at_unix_seconds FROM team_tactical_profiles WHERE team_id = ? ORDER BY created_at_unix_seconds ASC",
        &team_id.to_string(),
    )
    .await?;

    let mut results = Vec::with_capacity(profile_rows.len());
    for row in profile_rows {
        let value_rows = fetch_all_by_param::<TacticalInstructionValueRow>(
            pool,
            "SELECT id, team_tactical_profile_id, phase, instruction_key, value FROM tactical_instruction_values WHERE team_tactical_profile_id = ?",
            &row.id,
        )
        .await?;
        let instructions = row.to_domain(&value_rows)?;
        results.push((row, instructions));
    }

    Ok(results)
}

pub async fn get_profile_by_id(
    pool: &SqlitePool,
    id: Uuid,
) -> TacticsResult<Option<(TeamTacticalProfileRow, TeamInstructions)>> {
    let profile_row = fetch_optional_by_param::<TeamTacticalProfileRow>(
        pool,
        "SELECT id, team_id, name, is_active, created_at_unix_seconds FROM team_tactical_profiles WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    let profile_row = match profile_row {
        Some(r) => r,
        None => return Ok(None),
    };

    let value_rows = fetch_all_by_param::<TacticalInstructionValueRow>(
        pool,
        "SELECT id, team_tactical_profile_id, phase, instruction_key, value FROM tactical_instruction_values WHERE team_tactical_profile_id = ?",
        &profile_row.id,
    )
    .await?;

    let instructions = profile_row.to_domain(&value_rows)?;
    Ok(Some((profile_row, instructions)))
}

pub async fn insert_profile(
    pool: &SqlitePool,
    team_id: Uuid,
    name: &str,
    instructions: &TeamInstructions,
) -> TacticsResult<Uuid> {
    let profile_id = Uuid::new_v4();
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let mut tx = pool.begin().await?;

    sqlx::query(
        "INSERT INTO team_tactical_profiles (id, team_id, name, is_active, created_at_unix_seconds) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(profile_id.to_string())
    .bind(team_id.to_string())
    .bind(name)
    .bind(0i64)
    .bind(timestamp)
    .execute(&mut *tx)
    .await?;

    let entries: [(TacticalPhase, InstructionKey, f64); 12] = [
        (
            TacticalPhase::InPossession,
            InstructionKey::Mentality,
            instructions.in_possession().mentality().value(),
        ),
        (
            TacticalPhase::InPossession,
            InstructionKey::Tempo,
            instructions.in_possession().tempo().value(),
        ),
        (
            TacticalPhase::InPossession,
            InstructionKey::Width,
            instructions.in_possession().width().value(),
        ),
        (
            TacticalPhase::InPossession,
            InstructionKey::FlankBias,
            instructions.in_possession().flank_bias().value(),
        ),
        (
            TacticalPhase::InPossession,
            InstructionKey::Directness,
            instructions.in_possession().directness().value(),
        ),
        (
            TacticalPhase::InPossession,
            InstructionKey::Structure,
            instructions.in_possession().structure().value(),
        ),
        (
            TacticalPhase::OutOfPossession,
            InstructionKey::DefensiveLineHeight,
            instructions
                .out_of_possession()
                .defensive_line_height()
                .value(),
        ),
        (
            TacticalPhase::OutOfPossession,
            InstructionKey::Compactness,
            instructions.out_of_possession().compactness().value(),
        ),
        (
            TacticalPhase::OutOfPossession,
            InstructionKey::PressingIntensity,
            instructions
                .out_of_possession()
                .pressing_intensity()
                .value(),
        ),
        (
            TacticalPhase::OutOfPossession,
            InstructionKey::Aggression,
            instructions.out_of_possession().aggression().value(),
        ),
        (
            TacticalPhase::Transition,
            InstructionKey::CounterAttackIntensity,
            instructions
                .transition()
                .counter_attack_intensity()
                .value(),
        ),
        (
            TacticalPhase::Transition,
            InstructionKey::CounterPressIntensity,
            instructions.transition().counter_press_intensity().value(),
        ),
    ];

    for (phase, key, value) in entries {
        let value_id = Uuid::new_v4().to_string();
        let phase_code = tactical_phase_to_code(phase);
        let key_code = instruction_key_to_code(key);
        sqlx::query(
            "INSERT INTO tactical_instruction_values (id, team_tactical_profile_id, phase, instruction_key, value) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(value_id)
        .bind(profile_id.to_string())
        .bind(phase_code)
        .bind(key_code)
        .bind(value)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(profile_id)
}

pub async fn set_active(pool: &SqlitePool, profile_id: Uuid) -> TacticsResult<()> {
    let mut tx = pool.begin().await?;

    let profile_row = sqlx::query_as::<_, TeamTacticalProfileRow>(
        "SELECT id, team_id, name, is_active, created_at_unix_seconds FROM team_tactical_profiles WHERE id = ?",
    )
    .bind(profile_id.to_string())
    .fetch_optional(&mut *tx)
    .await?;

    let profile = match profile_row {
        Some(p) => p,
        None => {
            return Err(TacticsError::NotFound(format!(
                "Team tactical profile not found: {}",
                profile_id
            )))
        }
    };

    sqlx::query("UPDATE team_tactical_profiles SET is_active = 0 WHERE team_id = ?")
        .bind(&profile.team_id)
        .execute(&mut *tx)
        .await?;

    sqlx::query("UPDATE team_tactical_profiles SET is_active = 1 WHERE id = ?")
        .bind(profile_id.to_string())
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(())
}

pub async fn delete_profile(pool: &SqlitePool, profile_id: Uuid) -> TacticsResult<()> {
    sqlx::query("DELETE FROM team_tactical_profiles WHERE id = ?")
        .bind(profile_id.to_string())
        .execute(pool)
        .await?;
    Ok(())
}