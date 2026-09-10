use crate::error::{EngineError, EngineResult};
use crate::lineup_runtime::lineup::{Lineup, LineupAssignment};
use arlo_domain::{Formation, Player};
use arlo_tactics::TacticalLineup;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub fn hydrate(
    tactical_lineup: &TacticalLineup,
    formation: &Formation,
    roster: &[Player],
) -> EngineResult<Lineup> {
    let mut roster_map: HashMap<Uuid, Arc<Player>> = HashMap::with_capacity(roster.len());
    for p in roster {
        roster_map.insert(p.id(), Arc::new(p.clone()));
    }

    let mut assignments = Vec::with_capacity(tactical_lineup.assignments().len());

    for assignment in tactical_lineup.assignments() {
        let player = roster_map
            .get(&assignment.player_id())
            .cloned()
            .ok_or_else(|| EngineError::PlayerNotFound(assignment.player_id()))?;

        let slot = formation
            .slots()
            .get(assignment.formation_slot_index())
            .copied()
            .ok_or_else(|| EngineError::SlotCountMismatch {
                expected: formation.slots().len(),
                actual: assignment.formation_slot_index(),
            })?;

        assignments.push(LineupAssignment::new(
            assignment.formation_slot_index(),
            slot,
            player,
            assignment.slot_role(),
            *assignment.player_instructions(),
        ));
    }

    Lineup::from_assignments(formation.clone(), assignments)
}