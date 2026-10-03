use crate::error::{TacticsError, TacticsResult};
use crate::persistence::models::marking_code::parse_marking_assignment;
use crate::persistence::models::rows::{
    build_player_instructions, TacticalLineupSlotInstructionRow, TacticalLineupSlotRow,
};
use crate::SlotAssignment;
use arlo_domain::Formation;
use sqlx::SqlitePool;
use std::collections::HashMap;

pub(super) async fn assignments(
    pool: &SqlitePool,
    lineup_id: &str,
    formation: &Formation,
) -> TacticsResult<Vec<SlotAssignment>> {
    let slots = sqlx::query_as::<_, TacticalLineupSlotRow>(
        "SELECT id, tactical_lineup_id, slot_index, player_id, slot_role, marking_scheme, marking_target_position FROM tactical_lineup_slots WHERE tactical_lineup_id = ? ORDER BY slot_index ASC",
    ).bind(lineup_id).fetch_all(pool).await?;
    let instructions = sqlx::query_as::<_, TacticalLineupSlotInstructionRow>(
        "SELECT i.id, i.tactical_lineup_slot_id, i.phase, i.instruction_key, i.value FROM tactical_lineup_slots s JOIN tactical_lineup_slot_instructions i ON i.tactical_lineup_slot_id = s.id WHERE s.tactical_lineup_id = ? ORDER BY s.slot_index, i.instruction_key",
    ).bind(lineup_id).fetch_all(pool).await?;
    let mut by_slot: HashMap<String, Vec<TacticalLineupSlotInstructionRow>> = HashMap::new();
    for instruction in instructions {
        by_slot
            .entry(instruction.tactical_lineup_slot_id.clone())
            .or_default()
            .push(instruction);
    }
    slots
        .into_iter()
        .map(|slot| {
            let position = formation
                .slots()
                .get(slot.slot_index as usize)
                .ok_or_else(|| {
                    TacticsError::InvalidLineup(format!(
                        "Slot index {} out of bounds for formation {}",
                        slot.slot_index,
                        formation.id()
                    ))
                })?;
            let marking = parse_marking_assignment(
                slot.marking_scheme.as_deref(),
                slot.marking_target_position.as_deref(),
            )?;
            let instructions = build_player_instructions(
                by_slot.get(&slot.id).map_or(&[], Vec::as_slice),
                marking,
            )?;
            slot.to_domain(position.position(), instructions)
        })
        .collect()
}
