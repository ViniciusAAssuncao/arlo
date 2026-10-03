use crate::error::TacticsResult;
use crate::persistence::models::marking_code::marking_assignment_to_columns;
use crate::persistence::models::player_instruction_key_code::{
    player_instruction_key_to_code, PlayerInstructionKey,
};
use crate::persistence::models::slot_role_code::slot_role_to_code;
use crate::persistence::models::tactical_phase::TacticalPhase;
use crate::persistence::models::tactical_phase_code::tactical_phase_to_code;
use crate::TacticalLineup;
use arlo_db::repositories::batching::execute_batch_insert;
use sqlx::SqlitePool;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

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

    let mut slots = Vec::new();
    let mut instruction_rows = Vec::new();
    for assignment in lineup.assignments() {
        let slot_id = Uuid::new_v4().to_string();
        let role_code = slot_role_to_code(assignment.slot_role());
        let (marking_scheme, marking_target_position) = marking_assignment_to_columns(
            assignment
                .player_instructions()
                .out_of_possession()
                .marking(),
        );

        slots.push((
            slot_id.clone(),
            assignment.formation_slot_index() as i32,
            assignment.player_id().to_string(),
            role_code,
            marking_scheme,
            marking_target_position,
        ));

        let instructions = assignment.player_instructions();
        let entries: [(TacticalPhase, PlayerInstructionKey, f64); 7] = [
            (
                TacticalPhase::InPossession,
                PlayerInstructionKey::PositioningBias,
                instructions.in_possession().positioning_bias().value(),
            ),
            (
                TacticalPhase::InPossession,
                PlayerInstructionKey::InvolvementPriority,
                instructions.in_possession().involvement_priority().value(),
            ),
            (
                TacticalPhase::InPossession,
                PlayerInstructionKey::CreativeLicense,
                instructions.in_possession().creative_license().value(),
            ),
            (
                TacticalPhase::OutOfPossession,
                PlayerInstructionKey::EngagementBias,
                instructions.out_of_possession().engagement_bias().value(),
            ),
            (
                TacticalPhase::OutOfPossession,
                PlayerInstructionKey::DepthDiscipline,
                instructions.out_of_possession().depth_discipline().value(),
            ),
            (
                TacticalPhase::Transition,
                PlayerInstructionKey::TransitionUrgency,
                instructions.transition().transition_urgency().value(),
            ),
            (
                TacticalPhase::Transition,
                PlayerInstructionKey::ReleaseTempo,
                instructions.transition().release_tempo().value(),
            ),
        ];

        for (phase, key, value) in entries {
            let instr_id = Uuid::new_v4().to_string();
            let phase_code = tactical_phase_to_code(phase);
            let key_code = player_instruction_key_to_code(key);
            instruction_rows.push((instr_id, slot_id.clone(), phase_code, key_code, value));
        }
    }

    let lineup_id = lineup.id().to_string();
    execute_batch_insert(
        &mut tx,
        "tactical_lineup_slots",
        &[
            "id",
            "tactical_lineup_id",
            "slot_index",
            "player_id",
            "slot_role",
            "marking_scheme",
            "marking_target_position",
        ],
        &slots,
        |b, row| {
            b.push_bind(&row.0)
                .push_bind(&lineup_id)
                .push_bind(row.1)
                .push_bind(&row.2)
                .push_bind(row.3)
                .push_bind(&row.4)
                .push_bind(&row.5);
        },
    )
    .await?;
    execute_batch_insert(
        &mut tx,
        "tactical_lineup_slot_instructions",
        &[
            "id",
            "tactical_lineup_slot_id",
            "phase",
            "instruction_key",
            "value",
        ],
        &instruction_rows,
        |b, row| {
            b.push_bind(&row.0)
                .push_bind(&row.1)
                .push_bind(row.2)
                .push_bind(row.3)
                .push_bind(row.4);
        },
    )
    .await?;
    tx.commit().await?;
    Ok(())
}
