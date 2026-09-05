use crate::fatigue::state::FatigueState;
use crate::spatial::decision_vector::extract_attribute_value;
use crate::weighting::apply_saturation;
use arlo_domain::sport_constants::BASE_FATIGUE_CAPACITY_MIRIM;
use arlo_domain::{AttributeKey, Player};
use std::collections::HashMap;
use uuid::Uuid;

pub fn fatigue_multiplier(state: &FatigueState, stamina: f64, natural_fitness: f64) -> f64 {
    let fitness_factor = 0.4 + (stamina * 0.035) + (natural_fitness * 0.025);
    let effective_capacity = (BASE_FATIGUE_CAPACITY_MIRIM * fitness_factor).max(1.0);
    let ratio = state.cumulative_distance_mirim() / effective_capacity;
    let saturated = apply_saturation(ratio * 0.30, 0.15, 0.5);
    (1.0 - saturated).clamp(0.5, 1.0)
}

pub fn compute_player_fatigue_multiplier(
    player: &Player,
    fatigue_state: &FatigueState,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let stamina = extract_attribute_value(player, attribute_keys, AttributeKey::Stamina);
    let natural_fitness =
        extract_attribute_value(player, attribute_keys, AttributeKey::NaturalFitness);
    fatigue_multiplier(fatigue_state, stamina, natural_fitness)
}