use crate::attributes::PlayerAttributeTable;
use crate::lineup_runtime::calculate_player_contribution;
use crate::physical::PhysicalState;
use arlo_domain::{AttributeKey, Formation, FormationSlot, Player, Position};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

fn slot_demand_priority(slot: &FormationSlot) -> u8 {
    if slot.defensive_position() == Position::Goalguard
        || slot.position() == Position::Goalguard
        || slot.offensive_position() == Position::Goalguard
    {
        0
    } else if slot.position() == Position::Passer {
        1
    } else if slot.position() == Position::Artrine {
        2
    } else {
        3
    }
}

pub fn assign_players(
    formation: &Formation,
    roster: &[Player],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> Vec<(usize, Player)> {
    let slots = formation.slots();
    let mut slot_indices: Vec<usize> = (0..slots.len()).collect();

    slot_indices.sort_by_key(|&idx| (slot_demand_priority(&slots[idx]), idx));

    let mut allocated_ids = HashSet::with_capacity(slots.len());
    let mut assignments = Vec::with_capacity(slots.len());
    let initial_state = PhysicalState::initial();

    for slot_idx in slot_indices {
        let slot = &slots[slot_idx];
        let target_pos = slot.position();

        let best_player = roster
            .iter()
            .filter(|p| !allocated_ids.contains(&p.id()))
            .max_by(|a, b| {
                let table_a = PlayerAttributeTable::from_player(a, attribute_keys);
                let table_b = PlayerAttributeTable::from_player(b, attribute_keys);
                let score_a = calculate_player_contribution(a, &table_a, target_pos, &initial_state).value();
                let score_b = calculate_player_contribution(b, &table_b, target_pos, &initial_state).value();

                score_a
                    .partial_cmp(&score_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

        if let Some(player) = best_player {
            allocated_ids.insert(player.id());
            assignments.push((slot_idx, player.clone()));
        }
    }

    assignments.sort_by_key(|(idx, _)| *idx);
    assignments
}
