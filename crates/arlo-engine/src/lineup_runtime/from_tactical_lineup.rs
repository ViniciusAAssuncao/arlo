use crate::error::{EngineError, EngineResult};
use crate::lineup_runtime::lineup::{Lineup, LineupAssignment};
use arlo_domain::{Formation, Player};
use arlo_tactics::TacticalLineup;

pub fn hydrate(
    tactical_lineup: &TacticalLineup,
    formation: &Formation,
    roster: &[Player],
) -> EngineResult<Lineup> {
    let mut assignments = Vec::with_capacity(tactical_lineup.assignments().len());

    for assignment in tactical_lineup.assignments() {
        let player = roster
            .iter()
            .find(|p| p.id() == assignment.player_id())
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
            slot,
            player.clone(),
            assignment.slot_role(),
            *assignment.player_instructions(),
        ));
    }

    Lineup::from_assignments(formation.clone(), assignments)
}
