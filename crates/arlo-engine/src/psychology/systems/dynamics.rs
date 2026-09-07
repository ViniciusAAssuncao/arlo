use crate::physical::systems::degradation::calculate_physical_exhaustion;
use crate::physical::PhysicalState;
use crate::psychology::state::ImpulseState;
use crate::psychology::systems::baseline::calculate_player_impulse_baseline;
use crate::spatial::decision_vector::extract_attribute_value;
use arlo_domain::sport_constants::{impulse_floor_for_baseline, IMPULSE_SCALE_MAX};
use arlo_domain::{AttributeKey, Player};
use std::collections::HashMap;
use uuid::Uuid;

pub fn fatigue_depression(physical_state: &PhysicalState) -> f64 {
    let exhaustion = calculate_physical_exhaustion(physical_state);
    (1.0 - 0.40 * exhaustion).clamp(0.40, 1.0)
}

pub fn calculate_impulse_recovery_tau(stamina: f64, natural_fitness: f64) -> f64 {
    let norm_fitness =
        (natural_fitness.clamp(0.0, 20.0) * 0.6 + stamina.clamp(0.0, 20.0) * 0.4) / 20.0;
    let tau = 180.0 - (130.0 * norm_fitness);
    tau.clamp(40.0, 240.0)
}

pub fn calculate_player_impulse_recovery_tau(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let stamina = extract_attribute_value(player, attribute_keys, AttributeKey::Stamina);
    let natural_fitness =
        extract_attribute_value(player, attribute_keys, AttributeKey::NaturalFitness);
    calculate_impulse_recovery_tau(stamina, natural_fitness)
}

pub fn update_impulse_with_tau(
    state: &mut ImpulseState,
    baseline: f64,
    physical_state: &PhysicalState,
    dt_seconds: f64,
    tau: f64,
) {
    if dt_seconds <= 0.0 {
        return;
    }
    let floor = impulse_floor_for_baseline(baseline) * fatigue_depression(physical_state);
    let target = floor + (baseline - floor) * (1.0 - calculate_physical_exhaustion(physical_state));
    let decay_factor = (-dt_seconds / tau.max(1.0)).exp();
    let current_acc = state.accumulator();
    let new_acc = (target + (current_acc - target) * decay_factor)
        .max(floor)
        .clamp(0.0, IMPULSE_SCALE_MAX as f64);
    state.set_accumulator(new_acc);
}

pub fn update_impulse(
    state: &mut ImpulseState,
    baseline: f64,
    physical_state: &PhysicalState,
    dt_seconds: f64,
) {
    let default_tau = calculate_impulse_recovery_tau(10.0, 10.0);
    update_impulse_with_tau(state, baseline, physical_state, dt_seconds, default_tau);
}

pub fn update_player_impulse(
    state: &mut ImpulseState,
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    physical_state: &PhysicalState,
    dt_seconds: f64,
) {
    let baseline = calculate_player_impulse_baseline(player, attribute_keys);
    let tau = calculate_player_impulse_recovery_tau(player, attribute_keys);
    update_impulse_with_tau(state, baseline, physical_state, dt_seconds, tau);
}