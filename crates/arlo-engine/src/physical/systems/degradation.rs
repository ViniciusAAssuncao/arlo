use crate::physical::state::PhysicalState;
use crate::spatial::decision_vector::{
    extract_attribute_value, ACCELERATION_SPEED_SCALE, BASE_SPRINT_SPEED_METERS_PER_SEC,
    PACE_SPEED_SCALE,
};
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::Speed;
use std::collections::HashMap;
use uuid::Uuid;

pub fn is_physical_attribute(key: AttributeKey) -> bool {
    matches!(
        key,
        AttributeKey::Acceleration
            | AttributeKey::Pace
            | AttributeKey::Agility
            | AttributeKey::Balance
            | AttributeKey::Strength
            | AttributeKey::Stamina
            | AttributeKey::JumpingReach
            | AttributeKey::NaturalFitness
    )
}

pub fn is_cognitive_or_technical_attribute(key: AttributeKey) -> bool {
    !is_physical_attribute(key)
}

pub fn physical_attribute_modifier(state: &PhysicalState) -> f64 {
    let energy = state.energy().clamp(0.0, 1.0);
    let w_bal = state.w_prime_balance().clamp(0.0, 1.0);
    let mod_val = energy * (0.80 + 0.20 * w_bal);
    mod_val.clamp(0.20, 1.0)
}

pub fn cognitive_technical_modifier(state: &PhysicalState, concentration: f64) -> f64 {
    let norm_conc = (concentration.clamp(0.0, 20.0)) / 20.0;
    let critical_threshold = (0.45 - 0.20 * norm_conc).clamp(0.15, 0.60);
    let current_energy = state.energy() * (0.85 + 0.15 * state.w_prime_balance());

    if current_energy >= critical_threshold {
        let buffer = (current_energy - critical_threshold) / (1.0 - critical_threshold).max(1e-5);
        (0.96 + 0.04 * buffer).clamp(0.96, 1.0)
    } else {
        let deficit = (critical_threshold - current_energy) / critical_threshold.max(1e-5);
        let k = 3.5 + (1.0 - norm_conc) * 2.5;
        let decay = (-k * deficit).exp();
        (0.30 + 0.66 * decay).clamp(0.20, 0.96)
    }
}

pub fn attribute_degradation_modifier(
    key: AttributeKey,
    state: &PhysicalState,
    concentration: f64,
) -> f64 {
    if is_physical_attribute(key) {
        physical_attribute_modifier(state)
    } else {
        cognitive_technical_modifier(state, concentration)
    }
}

pub fn extract_effective_attribute_value(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    key: AttributeKey,
    state: &PhysicalState,
) -> f64 {
    let base_val = extract_attribute_value(player, attribute_keys, key);
    let concentration =
        extract_attribute_value(player, attribute_keys, AttributeKey::Concentration);
    let modifier = attribute_degradation_modifier(key, state, concentration);
    (base_val * modifier).clamp(0.0, 20.0)
}

pub fn calculate_effective_player_speed(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    state: &PhysicalState,
) -> Speed {
    let pace = extract_effective_attribute_value(player, attribute_keys, AttributeKey::Pace, state);
    let accel = extract_effective_attribute_value(
        player,
        attribute_keys,
        AttributeKey::Acceleration,
        state,
    );
    let speed_val =
        BASE_SPRINT_SPEED_METERS_PER_SEC + (pace * PACE_SPEED_SCALE) + (accel * ACCELERATION_SPEED_SCALE);
    Speed::new(speed_val)
}