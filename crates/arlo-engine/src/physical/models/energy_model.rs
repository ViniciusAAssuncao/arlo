use crate::physical::state::PhysicalState;
use crate::spatial::decision_vector::extract_attribute_value;
use crate::weighting::apply_saturation;
use arlo_domain::{AttributeKey, Player};
use std::collections::HashMap;
use uuid::Uuid;

pub fn physical_multiplier(state: &PhysicalState, stamina: f64, natural_fitness: f64) -> f64 {
    let base_energy = state.energy().clamp(0.0, 1.0);
    let fitness_factor = 0.5 + (stamina.clamp(0.0, 20.0) * 0.03) + (natural_fitness.clamp(0.0, 20.0) * 0.02);
    let anaerobic_factor = 0.7 + 0.3 * state.w_prime_balance().clamp(0.0, 1.0);
    let distance_ratio = state.cumulative_distance_mirim() / (12000.0 * fitness_factor);
    let distance_penalty = apply_saturation(distance_ratio * 0.30, 0.15, 0.5);
    ((base_energy * anaerobic_factor) - distance_penalty).clamp(0.5, 1.0)
}

pub fn compute_player_physical_multiplier(
    player: &Player,
    physical_state: &PhysicalState,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let stamina = extract_attribute_value(player, attribute_keys, AttributeKey::Stamina);
    let natural_fitness = extract_attribute_value(player, attribute_keys, AttributeKey::NaturalFitness);
    physical_multiplier(physical_state, stamina, natural_fitness)
}

pub fn fatigue_multiplier(state: &PhysicalState, stamina: f64, natural_fitness: f64) -> f64 {
    physical_multiplier(state, stamina, natural_fitness)
}

pub fn compute_player_fatigue_multiplier(
    player: &Player,
    physical_state: &PhysicalState,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    compute_player_physical_multiplier(player, physical_state, attribute_keys)
}
