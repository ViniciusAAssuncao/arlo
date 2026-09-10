use crate::attributes::PlayerAttributeTable;
use crate::physical::models::metabolic_power::{
    calculate_player_critical_speed, calculate_player_max_sprint_speed,
};
use crate::physical::state::PhysicalState;
use crate::psychology::state::ImpulseState;
use crate::spatial::decision_vector::extract_attribute_value;
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

pub fn calculate_physical_exhaustion(state: &PhysicalState) -> f64 {
    (1.0 - state.w_prime_balance()).max(0.0) * 0.65 + (1.0 - state.energy()).max(0.0) * 0.35
}

pub fn physical_attribute_modifier(state: &PhysicalState) -> f64 {
    let energy = state.energy().clamp(0.0, 1.0);
    let w_bal = state.w_prime_balance().clamp(0.0, 1.0);
    let combined = energy * (0.85 + 0.15 * w_bal);
    let mod_val = 0.70 + 0.30 * combined.powf(0.8);
    mod_val.clamp(0.40, 1.0)
}

pub fn cognitive_technical_modifier_with_impulse(
    state: &PhysicalState,
    concentration: f64,
    impulse_state: &ImpulseState,
    baseline: f64,
) -> f64 {
    let norm_conc = (concentration.clamp(0.0, 20.0)) / 20.0;
    let critical_threshold = (0.45 - 0.20 * norm_conc).clamp(0.15, 0.60);
    let current_energy = state.energy() * (0.85 + 0.15 * state.w_prime_balance());

    let base_mod = if current_energy >= critical_threshold {
        let buffer = (current_energy - critical_threshold) / (1.0 - critical_threshold).max(1e-5);
        (0.94 + 0.06 * buffer).clamp(0.94, 1.0)
    } else {
        let deficit = (critical_threshold - current_energy) / critical_threshold.max(1e-5);
        let k = 2.0 + (1.0 - norm_conc) * 1.5;
        let decay = (-k * deficit).exp();
        (0.60 + 0.34 * decay).clamp(0.50, 0.94)
    };

    let impulse_delta = impulse_state.accumulator() - baseline;
    let impulse_modifier = if impulse_delta >= 0.0 {
        let norm_excess = impulse_delta / 50.0;
        0.05 * (2.0 / (1.0 + (-2.5 * norm_excess).exp()) - 1.0)
    } else {
        let norm_deficit = (-impulse_delta) / 50.0;
        -0.12 * (2.0 / (1.0 + (-2.5 * norm_deficit).exp()) - 1.0)
    };

    (base_mod + impulse_modifier).clamp(0.40, 1.05)
}

pub fn cognitive_technical_modifier(state: &PhysicalState, concentration: f64) -> f64 {
    cognitive_technical_modifier_with_impulse(
        state,
        concentration,
        &ImpulseState::from_baseline(50.0),
        50.0,
    )
}

pub fn attribute_degradation_modifier_with_impulse(
    key: AttributeKey,
    state: &PhysicalState,
    concentration: f64,
    impulse_state: &ImpulseState,
    baseline: f64,
) -> f64 {
    if is_physical_attribute(key) {
        physical_attribute_modifier(state)
    } else {
        cognitive_technical_modifier_with_impulse(state, concentration, impulse_state, baseline)
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

pub fn extract_effective_attribute_value_with_impulse(
    table: &PlayerAttributeTable,
    key: AttributeKey,
    state: &PhysicalState,
    impulse_state: &ImpulseState,
    baseline: f64,
) -> f64 {
    let base_val = extract_attribute_value(table, key);
    let concentration = extract_attribute_value(table, AttributeKey::Concentration);
    let modifier = attribute_degradation_modifier_with_impulse(
        key,
        state,
        concentration,
        impulse_state,
        baseline,
    );
    (base_val * modifier).clamp(0.0, 20.0)
}

pub fn extract_effective_attribute_value(
    table: &PlayerAttributeTable,
    key: AttributeKey,
    state: &PhysicalState,
) -> f64 {
    let base_val = extract_attribute_value(table, key);
    let concentration = extract_attribute_value(table, AttributeKey::Concentration);
    let modifier = attribute_degradation_modifier(key, state, concentration);
    (base_val * modifier).clamp(0.0, 20.0)
}

pub fn calculate_effective_player_speed_with_impulse(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    state: &PhysicalState,
    impulse_state: &ImpulseState,
) -> Speed {
    let max_speed = calculate_player_max_sprint_speed(player, attribute_keys).value();
    let crit_speed = calculate_player_critical_speed(player, attribute_keys, 0).value();
    let w_bal = state.w_prime_balance().clamp(0.0, 1.0);
    let phys_mod = physical_attribute_modifier(state);
    let speed_ceiling = crit_speed + (max_speed - crit_speed).max(0.0) * w_bal;
    let _ = impulse_state;
    let final_speed = speed_ceiling * phys_mod;
    Speed::new(final_speed.max(1.0))
}

pub fn calculate_effective_player_speed(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    state: &PhysicalState,
) -> Speed {
    let max_speed = calculate_player_max_sprint_speed(player, attribute_keys).value();
    let crit_speed = calculate_player_critical_speed(player, attribute_keys, 0).value();
    let w_bal = state.w_prime_balance().clamp(0.0, 1.0);
    let phys_mod = physical_attribute_modifier(state);
    let speed_ceiling = crit_speed + (max_speed - crit_speed).max(0.0) * w_bal;
    let final_speed = speed_ceiling * phys_mod;
    Speed::new(final_speed.max(1.0))
}