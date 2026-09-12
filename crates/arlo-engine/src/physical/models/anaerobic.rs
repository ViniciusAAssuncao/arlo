use crate::attributes::PlayerAttributeTable;
use crate::physical::models::metabolic_power::{
    calculate_critical_speed as calc_crit_speed, calculate_max_w_prime as calc_max_w_prime,
    calculate_metabolic_work_rate, calculate_player_body_mass,
    calculate_player_body_mass_from_table,
    calculate_player_critical_speed as calc_player_crit_speed,
    calculate_player_max_w_prime as calc_player_max_w_prime, estimate_body_mass,
};
use crate::physical::state::PhysicalState;
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::resolution::DuelKind;
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::Speed;
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_max_w_prime(acceleration: f64, pace: f64, strength: f64) -> f64 {
    let mass = estimate_body_mass(1.82, strength);
    calc_max_w_prime(strength, (acceleration + pace * 0.3) / 1.3, mass)
}

pub fn calculate_player_max_w_prime(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    calc_player_max_w_prime(player, attribute_keys)
}

pub fn calculate_critical_speed(stamina: f64, natural_fitness: f64) -> f64 {
    calc_crit_speed(stamina, natural_fitness, 10.0, 25.0)
}

pub fn calculate_player_critical_speed(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    calc_player_crit_speed(player, attribute_keys, 0).value()
}

pub fn calculate_duel_intensity_multiplier(duel_kind: DuelKind) -> f64 {
    match duel_kind {
        DuelKind::FinishingAttempt => 2.5,
        DuelKind::ArtroBreakthrough => 2.2,
        DuelKind::RunBreakthrough => 2.0,
        DuelKind::CentralBlock | DuelKind::LateralBlock => 1.8,
        DuelKind::PassProtection | DuelKind::KickBlockAttempt => 1.7,
        DuelKind::RouteContest | DuelKind::AerialDuel => 1.6,
        DuelKind::BallSecurityCarry | DuelKind::BallSecurityDistribution => 1.5,
        DuelKind::ShortDistribution
        | DuelKind::LongDistribution
        | DuelKind::CrossDistribution
        | DuelKind::FieldGoalAttempt => 1.2,
    }
}

pub fn calculate_anaerobic_cost(
    duration_seconds: f64,
    speed_meters_per_sec: f64,
    critical_speed: f64,
    intensity_multiplier: f64,
) -> f64 {
    let duration = duration_seconds.max(0.0);
    let mass = 78.0;
    let rate = calculate_metabolic_work_rate(
        speed_meters_per_sec,
        critical_speed,
        mass,
        intensity_multiplier,
    );
    rate * duration
}

pub fn calculate_player_anaerobic_cost_from_table(
    player: &Player,
    table: &PlayerAttributeTable,
    duration_seconds: f64,
    speed_meters_per_sec: f64,
    critical_speed: f64,
    intensity_multiplier: f64,
) -> f64 {
    let duration = duration_seconds.max(0.0);
    let mass = calculate_player_body_mass_from_table(player, table);
    let rate = calculate_metabolic_work_rate(
        speed_meters_per_sec,
        critical_speed,
        mass,
        intensity_multiplier,
    );
    rate * duration
}

pub fn calculate_player_anaerobic_cost(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    duration_seconds: f64,
    speed_meters_per_sec: f64,
    critical_speed: f64,
    intensity_multiplier: f64,
) -> f64 {
    let duration = duration_seconds.max(0.0);
    let mass = calculate_player_body_mass(player, attribute_keys);
    let rate = calculate_metabolic_work_rate(
        speed_meters_per_sec,
        critical_speed,
        mass,
        intensity_multiplier,
    );
    rate * duration
}

pub fn apply_anaerobic_cost_to_state(state: &mut PhysicalState, cost: f64, max_w_prime: f64) {
    if cost <= 0.0 || max_w_prime <= 0.0 {
        return;
    }
    let current_w = state.w_prime_balance() * max_w_prime;
    let new_w = (current_w - cost).max(0.0);
    let new_balance = (new_w / max_w_prime).clamp(0.0, 1.0);
    state.set_w_prime_balance(new_balance);
    state.increment_high_intensity_actions();
}

pub fn calculate_player_effective_speed(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    physical_state: &PhysicalState,
) -> Speed {
    calculate_effective_player_speed(player, attribute_keys, physical_state)
}