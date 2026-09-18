use arlo_domain::{Player, Position as DomainPosition};
use arlo_tactics::{MarkingAssignment, PlayerInstructions};
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_lead_defender_with_marking<'a>(
    carrier_id: Uuid,
    offense_pos_index: &HashMap<Uuid, DomainPosition>,
    defenders: &[&'a Player],
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
) -> &'a Player {
    if let Some(&carrier_position) = offense_pos_index.get(&carrier_id) {
        for &defender in defenders {
            if let Some(instructions) = instructions_index.get(&defender.id()) {
                if let Some(MarkingAssignment::Man(target)) =
                    instructions.out_of_possession().marking()
                {
                    if target == carrier_position {
                        return defender;
                    }
                }
            }
        }
    }

    defenders.first().copied().unwrap()
}
