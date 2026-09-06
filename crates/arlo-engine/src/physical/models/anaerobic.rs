use crate::physical::state::PhysicalState;
use crate::resolution::DuelKind;
use crate::spatial::decision_vector::{
    calculate_player_speed, extract_attribute_value, BASE_SPRINT_SPEED_METERS_PER_SEC,
};
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::Speed;
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_max_w_prime(acceleration: f64, pace: f64, strength: f64) -> f64 {
    let accel = acceleration.clamp(0.0, 20.0);
    let p = pace.clamp(0.0, 20.0);
    let s = strength.clamp(0.0, 20.0);
    400.0 + (accel * 35.0) + (p * 25.0) + (s * 20.0)
}

pub fn calculate_player_max_w_prime(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let accel = extract_attribute_value(player, attribute_keys, AttributeKey::Acceleration);
    let pace = extract_attribute_value(player, attribute_keys, AttributeKey::Pace);
    let strength = extract_attribute_value(player, attribute_keys, AttributeKey::Strength);
    calculate_max_w_prime(accel, pace, strength)
}

pub fn calculate_critical_speed(stamina: f64, natural_fitness: f64) -> f64 {
    let st = stamina.clamp(0.0, 20.0);
    let nf = natural_fitness.clamp(0.0, 20.0);
    BASE_SPRINT_SPEED_METERS_PER_SEC * 0.60 + (st * 0.08) + (nf * 0.06)
}

pub fn calculate_player_critical_speed(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let stamina = extract_attribute_value(player, attribute_keys, AttributeKey::Stamina);
    let natural_fitness =
        extract_attribute_value(player, attribute_keys, AttributeKey::NaturalFitness);
    calculate_critical_speed(stamina, natural_fitness)
}

pub fn calculate_duel_intensity_multiplier(duel_kind: DuelKind) -> f64 {
    match duel_kind {
        DuelKind::FinishingAttempt => 2.5,
        DuelKind::ArtroBreakthrough => 2.2,
        DuelKind::RunBreakthrough => 2.0,
        DuelKind::CentralBlock | DuelKind::LateralBlock => 1.8,
        DuelKind::PassProtection => 1.7,
        DuelKind::RouteContest | DuelKind::AerialDuel => 1.6,
        DuelKind::BallSecurityCarry | DuelKind::BallSecurityDistribution => 1.5,
        DuelKind::ShortDistribution | DuelKind::LongDistribution | DuelKind::CrossDistribution => {
            1.2
        }
    }
}

pub fn calculate_anaerobic_cost(
    duration_seconds: f64,
    speed_meters_per_sec: f64,
    critical_speed: f64,
    intensity_multiplier: f64,
) -> f64 {
    let duration = duration_seconds.max(0.0);
    let speed_delta = (speed_meters_per_sec - critical_speed).max(0.0);
    let movement_cost = duration * speed_delta * 4.0;
    let duel_cost = if intensity_multiplier > 1.0 {
        duration * (intensity_multiplier - 1.0) * 6.0
    } else {
        0.0
    };
    (movement_cost + duel_cost) * intensity_multiplier.max(1.0)
}

pub fn apply_anaerobic_cost_to_state(
    state: &mut PhysicalState,
    cost: f64,
    max_w_prime: f64,
) {
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
    let max_speed = calculate_player_speed(player, attribute_keys, physical_state.energy());
    let critical_speed = calculate_player_critical_speed(player, attribute_keys);
    let w_balance = physical_state.w_prime_balance().clamp(0.0, 1.0);
    let effective_speed_val =
        critical_speed + (max_speed.value() - critical_speed).max(0.0) * w_balance;
    Speed::new(effective_speed_val.min(max_speed.value()))
}