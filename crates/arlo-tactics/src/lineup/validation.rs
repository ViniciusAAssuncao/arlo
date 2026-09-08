use crate::error::{TacticsError, TacticsResult};
use crate::instructions::player::MarkingAssignment;
use crate::lineup::special_role_rules::{is_role_eligible_for_position, max_concurrent_count};
use crate::lineup::tactical_lineup::TacticalLineup;
use arlo_domain::{Formation, Player, SlotRole};
use std::collections::{HashMap, HashSet};

pub fn validate_tactical_lineup(
    lineup: &TacticalLineup,
    formation: &Formation,
    roster: &[Player],
) -> TacticsResult<()> {
    if lineup.assignments().len() != formation.slots().len() {
        return Err(TacticsError::InvalidLineup(format!(
            "Slot count mismatch: expected {}, found {}",
            formation.slots().len(),
            lineup.assignments().len()
        )));
    }

    let mut seen_players = HashSet::new();
    for assignment in lineup.assignments() {
        if !seen_players.insert(assignment.player_id()) {
            return Err(TacticsError::InvalidLineup(format!(
                "Duplicate player assignment: {}",
                assignment.player_id()
            )));
        }
    }

    for assignment in lineup.assignments() {
        let player = roster
            .iter()
            .find(|p| p.id() == assignment.player_id())
            .ok_or_else(|| {
                TacticsError::InvalidLineup(format!(
                    "Player {} not found in roster",
                    assignment.player_id()
                ))
            })?;

        let has_position = player
            .positions()
            .iter()
            .any(|p| p.position() == assignment.position());

        if !has_position {
            return Err(TacticsError::InvalidLineup(format!(
                "Player {} does not have required position {:?} registered",
                player.id(),
                assignment.position()
            )));
        }
    }

    let mut role_counts: HashMap<SlotRole, u32> = HashMap::new();
    for assignment in lineup.assignments() {
        *role_counts.entry(assignment.slot_role()).or_insert(0) += 1;
    }

    for (&role, &count) in &role_counts {
        if let Some(max) = max_concurrent_count(role) {
            if count > max {
                return Err(TacticsError::InvalidLineup(format!(
                    "Role {:?} exceeds maximum concurrent count of {} (found {})",
                    role, max, count
                )));
            }
        }
    }

    for assignment in lineup.assignments() {
        if assignment.slot_role() != SlotRole::Standard
            && !is_role_eligible_for_position(assignment.slot_role(), assignment.position())
        {
            return Err(TacticsError::InvalidLineup(format!(
                "Role {:?} is not eligible for position {:?}",
                assignment.slot_role(),
                assignment.position()
            )));
        }
    }

    for assignment in lineup.assignments() {
        if let Some(MarkingAssignment::Man(target)) = assignment
            .player_instructions()
            .out_of_possession()
            .marking()
        {
            if target == assignment.position() {
                return Err(TacticsError::InvalidLineup(
                    "player cannot man-mark their own position".to_string(),
                ));
            }
        }
    }

    Ok(())
}
