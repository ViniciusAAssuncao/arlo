use crate::spatial::decision_vector::{calculate_player_speed, extract_attribute_value};
use crate::weighting::apply_saturation;
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::{Position, Speed, Velocity};
use std::collections::HashMap;
use std::f64::consts::PI;
use uuid::Uuid;

pub fn max_turn_radians_per_tick(agility: f64) -> f64 {
    let clamped_agility = agility.clamp(0.0, 20.0);
    let base_turn = 0.10 + clamped_agility * 0.025;
    apply_saturation(base_turn, 0.45, 0.5)
}

pub fn calculate_steered_velocity(
    current_velocity: Velocity,
    current_pos: Position,
    target_pos: Position,
    speed: Speed,
    agility: f64,
) -> Velocity {
    let delta = target_pos.raw() - current_pos.raw();
    let dist = delta.magnitude();
    if dist < 1e-6 {
        return Velocity::zero();
    }

    let target_dir = delta / dist;
    let current_speed = current_velocity.magnitude().value();

    if current_speed < 1e-6 {
        let raw_vel = target_dir * speed.value();
        return Velocity::from_raw(raw_vel);
    }

    let current_dir = current_velocity.raw() / current_speed;
    let current_angle = current_dir.1.atan2(current_dir.0);
    let target_angle = target_dir.1.atan2(target_dir.0);

    let mut diff = target_angle - current_angle;
    while diff > PI {
        diff -= 2.0 * PI;
    }
    while diff < -PI {
        diff += 2.0 * PI;
    }

    let max_turn = max_turn_radians_per_tick(agility);
    let applied_diff = diff.clamp(-max_turn, max_turn);
    let new_angle = current_angle + applied_diff;

    let new_dir_raw = Position::from_components(new_angle.cos(), new_angle.sin(), 0.0).raw();
    let raw_vel = new_dir_raw * speed.value();
    Velocity::from_raw(raw_vel)
}

pub fn derive_player_steered_velocity(
    current_velocity: Velocity,
    current_pos: Position,
    target_pos: Position,
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_multiplier: f64,
) -> Velocity {
    let speed = calculate_player_speed(player, attribute_keys, fatigue_multiplier);
    let agility = extract_attribute_value(player, attribute_keys, AttributeKey::Agility);
    calculate_steered_velocity(current_velocity, current_pos, target_pos, speed, agility)
}
