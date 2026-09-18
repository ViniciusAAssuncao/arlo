use crate::attributes::PlayerAttributeTable;
use crate::current_ability::calculate_player_ca;
use crate::lineup_runtime::fit_calculator::calculate_fit;
use arlo_domain::sport_constants::MIN_CURRENT_ABILITY;
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

    for slot_idx in slot_indices {
        let slot = &slots[slot_idx];

        let best_player = roster
            .iter()
            .filter(|p| !allocated_ids.contains(&p.id()))
            .max_by(|a, b| {
                let fit_a = calculate_fit(a, slot);
                let fit_b = calculate_fit(b, slot);

                let table_a = PlayerAttributeTable::from_player(a, attribute_keys);
                let table_b = PlayerAttributeTable::from_player(b, attribute_keys);
                let ca_a = calculate_player_ca(a, &table_a).unwrap_or(MIN_CURRENT_ABILITY) as f64;
                let ca_b = calculate_player_ca(b, &table_b).unwrap_or(MIN_CURRENT_ABILITY) as f64;

                let score_a = ca_a * (0.30 + 0.70 * fit_a.efficiency_multiplier());
                let score_b = ca_b * (0.30 + 0.70 * fit_b.efficiency_multiplier());

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