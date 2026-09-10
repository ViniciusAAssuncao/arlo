use crate::caching::position_profile_cache::get_position_profile;
use crate::current_ability::calculator::calculate_current_ability;
use arlo_domain::{AttributeKey, Player};
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_player_ca(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> Option<i32> {
    let best_position = player
        .positions()
        .iter()
        .max_by_key(|pos| pos.proficiency())?;

    let profile = get_position_profile(best_position.position());

    let mut attributes_and_weights = Vec::with_capacity(profile.weights.len());

    for attr in player.attributes() {
        if let Some(&key) = attribute_keys.get(&attr.attribute_definition_id()) {
            if let Some(weight) = profile.dense_weights[key.index()] {
                attributes_and_weights.push((attr.value() as f64, weight));
            }
        }
    }

    Some(calculate_current_ability(&attributes_and_weights))
}