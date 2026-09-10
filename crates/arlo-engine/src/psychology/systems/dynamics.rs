use crate::attributes::PlayerAttributeTable;
use crate::physical::systems::degradation::calculate_physical_exhaustion;
use crate::physical::PhysicalState;
use crate::psychology::state::ImpulseState;
use crate::psychology::systems::baseline::{
    calculate_player_contextual_baseline, calculate_player_contextual_baseline_from_table,
};
use crate::spatial::decision_vector::extract_attribute_value;
use arlo_domain::sport_constants::{impulse_floor_for_baseline, IMPULSE_SCALE_MAX};
use arlo_domain::{AttributeKey, Player};
use std::collections::HashMap;
use uuid::Uuid;

pub fn fatigue_depression(physical_state: &PhysicalState) -> f64 {
    let exhaustion = calculate_physical_exhaustion(physical_state);
    (1.0 - 0.4 * exhaustion).clamp(0.4, 1.0)
}

pub fn calculate_impulse_recovery_tau(stamina: f64, natural_fitness: f64) -> f64 {
    let norm_fitness =
        (natural_fitness.clamp(0.0, 20.0) * 0.6 + stamina.clamp(0.0, 20.0) * 0.4) / 20.0;
    let tau = 180.0 - 130.0 * norm_fitness;
    tau.clamp(40.0, 240.0)
}

pub fn calculate_player_impulse_recovery_tau_from_table(
    table: &PlayerAttributeTable,
) -> f64 {
    let stamina = extract_attribute_value(table, AttributeKey::Stamina);
    let natural_fitness = extract_attribute_value(table, AttributeKey::NaturalFitness);
    calculate_impulse_recovery_tau(stamina, natural_fitness)
}

pub fn calculate_player_impulse_recovery_tau(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    calculate_player_impulse_recovery_tau_from_table(&table)
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

pub fn update_player_impulse_contextual_from_table(
    state: &mut ImpulseState,
    player: &Player,
    table: &PlayerAttributeTable,
    physical_state: &PhysicalState,
    dt_seconds: f64,
    captain_influence: f64,
    is_captain: bool,
    is_home: bool,
) {
    let baseline = calculate_player_contextual_baseline_from_table(
        player,
        table,
        captain_influence,
        is_captain,
        is_home,
    );
    let tau = calculate_player_impulse_recovery_tau_from_table(table);
    let effective_tau = if is_home { tau * 0.95 } else { tau };
    update_impulse_with_tau(state, baseline, physical_state, dt_seconds, effective_tau);
}

pub fn update_player_impulse_contextual(
    state: &mut ImpulseState,
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    physical_state: &PhysicalState,
    dt_seconds: f64,
    captain: Option<&Player>,
    is_home: bool,
) {
    let baseline = calculate_player_contextual_baseline(player, attribute_keys, captain, is_home);
    let tau = calculate_player_impulse_recovery_tau(player, attribute_keys);
    let effective_tau = if is_home { tau * 0.95 } else { tau };
    update_impulse_with_tau(state, baseline, physical_state, dt_seconds, effective_tau);
}

pub fn update_player_impulse(
    state: &mut ImpulseState,
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    physical_state: &PhysicalState,
    dt_seconds: f64,
) {
    update_player_impulse_contextual(
        state,
        player,
        attribute_keys,
        physical_state,
        dt_seconds,
        None,
        false,
    );
}
