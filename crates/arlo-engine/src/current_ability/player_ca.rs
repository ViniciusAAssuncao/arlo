use crate::current_ability::calculator::calculate_current_ability;
use crate::current_ability::profiles::get_profile_for_position;
use arlo_domain::Player;
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_player_ca(
    player: &Player,
    attribute_keys: &HashMap<Uuid, String>,
) -> Option<i32> {
    let best_position = player
        .positions()
        .iter()
        .max_by_key(|pos| pos.proficiency())?;

    let profile = get_profile_for_position(best_position.position());

    let mut attributes_and_weights = Vec::new();

    for attr in player.attributes() {
        if let Some(key) = attribute_keys.get(&attr.attribute_definition_id()) {
            if let Some(weight) = profile.weights.iter().find(|w| &w.key == key) {
                if weight.weight > 0.0 {
                    attributes_and_weights.push((attr.value() as f64, weight.weight));
                }
            }
        }
    }

    Some(calculate_current_ability(&attributes_and_weights))
}