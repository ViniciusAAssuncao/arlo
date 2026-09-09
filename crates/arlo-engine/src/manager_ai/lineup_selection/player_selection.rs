use crate::current_ability::calculate_player_ca;
use crate::lineup_runtime::fit_calculator::calculate_fit_for_position;
use arlo_domain::{AttributeKey, Formation, Player, Position};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

fn position_demand_priority(pos: Position) -> u8 {
    match pos {
        Position::Goalguard => 0,
        Position::Passer => 1,
        Position::Artrine => 2,
        _ => 3,
    }
}

pub fn assign_players(
    formation: &Formation,
    roster: &[Player],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> Vec<(usize, Player)> {
    let slots = formation.slots();
    let mut slot_indices: Vec<usize> = (0..slots.len()).collect();

    slot_indices.sort_by_key(|&idx| (position_demand_priority(slots[idx].position()), idx));

    let mut allocated_ids = HashSet::with_capacity(slots.len());
    let mut assignments = Vec::with_capacity(slots.len());

    for slot_idx in slot_indices {
        let slot = &slots[slot_idx];
        let target_pos = slot.position();

        let best_player = roster
            .iter()
            .filter(|p| !allocated_ids.contains(&p.id()))
            .max_by(|a, b| {
                let fit_a = calculate_fit_for_position(a, target_pos);
                let fit_b = calculate_fit_for_position(b, target_pos);

                let prof_cmp = fit_a
                    .effective_proficiency()
                    .partial_cmp(&fit_b.effective_proficiency())
                    .unwrap_or(std::cmp::Ordering::Equal);

                if prof_cmp != std::cmp::Ordering::Equal {
                    prof_cmp
                } else {
                    let ca_a = calculate_player_ca(a, attribute_keys);
                    let ca_b = calculate_player_ca(b, attribute_keys);
                    ca_a.partial_cmp(&ca_b).unwrap_or(std::cmp::Ordering::Equal)
                }
            });

        if let Some(player) = best_player {
            allocated_ids.insert(player.id());
            assignments.push((slot_idx, player.clone()));
        }
    }

    assignments.sort_by_key(|(idx, _)| *idx);
    assignments
}