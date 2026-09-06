use crate::spatial::decision_vector::{calculate_player_speed, extract_attribute_value};
use crate::weighting::apply_saturation;
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::{Duration, Position, Speed, Vector3, Velocity, MIRIM_TO_METERS};
use std::collections::HashMap;
use std::f64::consts::PI;
use uuid::Uuid;

pub const DEFAULT_SEPARATION_RADIUS_METERS: f64 = 2.5 * MIRIM_TO_METERS;
pub const DEFAULT_ARRIVAL_SLOWING_RADIUS_METERS: f64 = 2.0 * MIRIM_TO_METERS;

pub fn max_turn_radians_per_tick(agility: f64) -> f64 {
    let clamped_agility = agility.clamp(0.0, 20.0);
    let base_turn = 0.10 + clamped_agility * 0.025;
    apply_saturation(base_turn, 0.45, 0.5)
}

pub fn calculate_seek_force(
    current_pos: Position,
    current_velocity: Velocity,
    target_pos: Position,
    max_speed: Speed,
    arrival_radius_meters: f64,
) -> Vector3 {
    let delta = target_pos.raw() - current_pos.raw();
    let dist = delta.magnitude();
    if dist < 1e-6 {
        return -current_velocity.raw();
    }

    let desired_speed = if dist < arrival_radius_meters && arrival_radius_meters > 1e-6 {
        max_speed.value() * (dist / arrival_radius_meters)
    } else {
        max_speed.value()
    };

    let desired_vel = (delta / dist) * desired_speed;
    desired_vel - current_velocity.raw()
}

pub fn calculate_separation_force(
    current_pos: Position,
    neighbor_positions: &[Position],
    separation_radius_meters: f64,
) -> Vector3 {
    let mut total_repulsion = Vector3::zero();
    let p0 = current_pos.raw();

    for &neighbor in neighbor_positions {
        let pn = neighbor.raw();
        let diff = p0 - pn;
        let dist = diff.magnitude();

        if dist > 1e-4 && dist < separation_radius_meters {
            let normalized_dist = dist / separation_radius_meters;
            let weight = (1.0 - normalized_dist) / dist;
            total_repulsion = total_repulsion + (diff * weight);
        }
    }

    total_repulsion
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

pub fn calculate_boid_steering_velocity(
    current_velocity: Velocity,
    current_pos: Position,
    target_pos: Position,
    neighbor_positions: &[Position],
    speed: Speed,
    agility: f64,
    acceleration: f64,
    dt: Duration,
) -> Velocity {
    let seek_force = calculate_seek_force(
        current_pos,
        current_velocity,
        target_pos,
        speed,
        DEFAULT_ARRIVAL_SLOWING_RADIUS_METERS,
    );

    let separation_force = calculate_separation_force(
        current_pos,
        neighbor_positions,
        DEFAULT_SEPARATION_RADIUS_METERS,
    );

    let max_accel_mag = 2.5 + (acceleration.clamp(0.0, 20.0) * 0.25);
    let combined_force = seek_force + (separation_force * 3.0);
    let force_mag = combined_force.magnitude();

    let clamped_force = if force_mag > max_accel_mag && force_mag > 1e-6 {
        (combined_force / force_mag) * max_accel_mag
    } else {
        combined_force
    };

    let candidate_raw = current_velocity.raw() + (clamped_force * dt.value());
    let candidate_speed = candidate_raw.magnitude();

    if candidate_speed < 1e-6 {
        return Velocity::zero();
    }

    let max_spd = speed.value();
    let limited_raw = if candidate_speed > max_spd {
        (candidate_raw / candidate_speed) * max_spd
    } else {
        candidate_raw
    };

    let current_speed = current_velocity.magnitude().value();
    if current_speed < 1e-6 {
        return Velocity::from_raw(limited_raw);
    }

    let current_dir = current_velocity.raw() / current_speed;
    let candidate_dir = limited_raw / limited_raw.magnitude();

    let current_angle = current_dir.1.atan2(current_dir.0);
    let target_angle = candidate_dir.1.atan2(candidate_dir.0);

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

    let final_dir = Position::from_components(new_angle.cos(), new_angle.sin(), 0.0).raw();
    let final_raw = final_dir * limited_raw.magnitude();
    Velocity::from_raw(final_raw)
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

pub fn derive_player_boid_steered_velocity(
    current_velocity: Velocity,
    current_pos: Position,
    target_pos: Position,
    neighbor_positions: &[Position],
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_multiplier: f64,
    dt: Duration,
) -> Velocity {
    let speed = calculate_player_speed(player, attribute_keys, fatigue_multiplier);
    let agility = extract_attribute_value(player, attribute_keys, AttributeKey::Agility);
    let acceleration = extract_attribute_value(player, attribute_keys, AttributeKey::Acceleration);
    calculate_boid_steering_velocity(
        current_velocity,
        current_pos,
        target_pos,
        neighbor_positions,
        speed,
        agility,
        acceleration,
        dt,
    )
}
