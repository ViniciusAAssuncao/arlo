use crate::attributes::PlayerAttributeTable;
use crate::caching::position_profile_cache::get_position_profile;
use crate::current_ability::calculator::calculate_current_ability;
use arlo_domain::{AttributeKey, Player};
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_player_ca_from_table(
    player: &Player,
    table: &PlayerAttributeTable,
) -> Option<i32> {
    let best_position = player
        .positions()
        .iter()
        .max_by_key(|pos| pos.proficiency())?;

    let profile = get_position_profile(best_position.position());

    let mut attributes_and_weights = Vec::with_capacity(profile.weights.len());

    for w in &profile.weights {
        if w.weight > 0.0 {
            attributes_and_weights.push((table.get(w.key), w.weight));
        }
    }

    Some(calculate_current_ability(&attributes_and_weights))
}

pub fn calculate_player_ca(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> Option<i32> {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    calculate_player_ca_from_table(player, &table)
}
