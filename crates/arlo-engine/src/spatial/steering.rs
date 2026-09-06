use crate::spatial::decision_vector::{calculate_player_speed, extract_attribute_value};
use crate::weighting::apply_saturation;
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::{Duration, Position, Speed, Vector3, Velocity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::f64::consts::PI;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SpatialNeighbor {
    pub id: Uuid,
    pub position: Position,
    pub velocity: Velocity,
    pub is_teammate: bool,
    pub physical_radius: f64,
}

impl SpatialNeighbor {
    pub fn new(
        id: Uuid,
        position: Position,
        velocity: Velocity,
        is_teammate: bool,
        physical_radius: f64,
    ) -> Self {
        Self {
            id,
            position,
            velocity,
            is_teammate,
            physical_radius,
        }
    }

    pub fn from_position(position: Position) -> Self {
        Self {
            id: Uuid::nil(),
            position,
            velocity: Velocity::zero(),
            is_teammate: false,
            physical_radius: 0.55,
        }
    }
}

pub fn derive_braking_deceleration(agility: f64, balance: f64, fatigue_multiplier: f64) -> f64 {
    let ag = agility.clamp(0.0, 20.0);
    let bal = balance.clamp(0.0, 20.0);
    let base_decel = 3.2 + (ag * 0.18) + (bal * 0.14);
    (base_decel * fatigue_multiplier.clamp(0.5, 1.0)).clamp(1.5, 12.0)
}

pub fn derive_arrival_slowing_radius(
    current_speed: f64,
    agility: f64,
    balance: f64,
    fatigue_multiplier: f64,
) -> f64 {
    let decel = derive_braking_deceleration(agility, balance, fatigue_multiplier);
    let speed = current_speed.max(0.0);
    let stopping_dist = (speed * speed) / (2.0 * decel);
    stopping_dist.max(0.35)
}

pub fn derive_player_arrival_radius(
    player: &Player,
    current_speed: f64,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_multiplier: f64,
) -> f64 {
    let agility = extract_attribute_value(player, attribute_keys, AttributeKey::Agility);
    let balance = extract_attribute_value(player, attribute_keys, AttributeKey::Balance);
    derive_arrival_slowing_radius(current_speed, agility, balance, fatigue_multiplier)
}

pub fn derive_player_physical_radius(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let height = player.height_m().clamp(1.4, 2.3);
    let strength = extract_attribute_value(player, attribute_keys, AttributeKey::Strength).clamp(0.0, 20.0);
    height * (0.22 + 0.008 * strength)
}

pub fn derive_dynamic_separation_radius(
    self_physical_radius: f64,
    neighbor_physical_radius: f64,
    is_teammate: bool,
    approach_speed: f64,
) -> f64 {
    let combined_radius = self_physical_radius + neighbor_physical_radius;
    let relational_factor = if is_teammate { 1.35 } else { 2.40 };
    let base_separation = combined_radius * relational_factor;
    let velocity_expansion = 1.0 + (approach_speed.max(0.0) * 0.15);
    base_separation * velocity_expansion
}

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

pub fn calculate_dynamic_separation_force(
    current_pos: Position,
    current_velocity: Velocity,
    self_physical_radius: f64,
    neighbors: &[SpatialNeighbor],
) -> Vector3 {
    calculate_dynamic_separation_force_with_id(
        current_pos,
        current_velocity,
        self_physical_radius,
        Uuid::nil(),
        true,
        neighbors,
    )
}

pub fn calculate_dynamic_separation_force_with_id(
    current_pos: Position,
    current_velocity: Velocity,
    self_physical_radius: f64,
    self_id: Uuid,
    self_is_home: bool,
    neighbors: &[SpatialNeighbor],
) -> Vector3 {
    let mut total_repulsion = Vector3::zero();
    let p0 = current_pos.raw();
    let v0 = current_velocity.raw();
    let v0_mag = v0.magnitude();

    for neighbor in neighbors {
        if neighbor.id == self_id && !self_id.is_nil() {
            continue;
        }
        let pn = neighbor.position.raw();
        let diff = p0 - pn;
        let dist = diff.magnitude();

        if dist > 1e-4 {
            let dir_to_neighbor = (pn - p0) / dist;
            let approach_speed = if v0_mag > 1e-6 {
                (v0.0 * dir_to_neighbor.0 + v0.1 * dir_to_neighbor.1 + v0.2 * dir_to_neighbor.2).max(0.0)
            } else {
                0.0
            };

            let is_teammate = neighbor.is_teammate == self_is_home;
            let sep_radius = derive_dynamic_separation_radius(
                self_physical_radius,
                neighbor.physical_radius,
                is_teammate,
                approach_speed,
            );

            if dist < sep_radius {
                let normalized_dist = dist / sep_radius;
                let weight = (1.0 - normalized_dist) / dist;
                total_repulsion = total_repulsion + (diff * weight);
            }
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

pub fn calculate_dynamic_boid_steering_velocity(
    current_velocity: Velocity,
    current_pos: Position,
    target_pos: Position,
    neighbors: &[SpatialNeighbor],
    speed: Speed,
    agility: f64,
    acceleration: f64,
    balance: f64,
    fatigue_multiplier: f64,
    self_physical_radius: f64,
    dt: Duration,
) -> Velocity {
    calculate_dynamic_boid_steering_velocity_with_id(
        current_velocity,
        current_pos,
        target_pos,
        Uuid::nil(),
        true,
        neighbors,
        speed,
        agility,
        acceleration,
        balance,
        fatigue_multiplier,
        self_physical_radius,
        dt,
    )
}

pub fn calculate_dynamic_boid_steering_velocity_with_id(
    current_velocity: Velocity,
    current_pos: Position,
    target_pos: Position,
    self_id: Uuid,
    self_is_home: bool,
    neighbors: &[SpatialNeighbor],
    speed: Speed,
    agility: f64,
    acceleration: f64,
    balance: f64,
    fatigue_multiplier: f64,
    self_physical_radius: f64,
    dt: Duration,
) -> Velocity {
    let current_speed = current_velocity.magnitude().value();
    let arrival_radius = derive_arrival_slowing_radius(
        current_speed.max(speed.value()),
        agility,
        balance,
        fatigue_multiplier,
    );

    let seek_force = calculate_seek_force(
        current_pos,
        current_velocity,
        target_pos,
        speed,
        arrival_radius,
    );

    let separation_force = calculate_dynamic_separation_force_with_id(
        current_pos,
        current_velocity,
        self_physical_radius,
        self_id,
        self_is_home,
        neighbors,
    );

    let dist_to_target = (target_pos.raw() - current_pos.raw()).magnitude();
    let sep_scale = if arrival_radius > 1e-4 {
        (dist_to_target / arrival_radius).clamp(0.0, 1.0)
    } else {
        1.0
    };

    let max_accel_mag = (2.5 + (acceleration.clamp(0.0, 20.0) * 0.25)) * fatigue_multiplier.clamp(0.5, 1.0);
    let combined_force = seek_force + (separation_force * (1.5 * sep_scale));
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
    let neighbors: Vec<SpatialNeighbor> = neighbor_positions
        .iter()
        .map(|&pos| SpatialNeighbor::from_position(pos))
        .collect();
    calculate_dynamic_boid_steering_velocity(
        current_velocity,
        current_pos,
        target_pos,
        &neighbors,
        speed,
        agility,
        acceleration,
        10.0,
        1.0,
        0.55,
        dt,
    )
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
    let balance = extract_attribute_value(player, attribute_keys, AttributeKey::Balance);
    let physical_radius = derive_player_physical_radius(player, attribute_keys);

    let neighbors: Vec<SpatialNeighbor> = neighbor_positions
        .iter()
        .map(|&pos| SpatialNeighbor::from_position(pos))
        .collect();

    calculate_dynamic_boid_steering_velocity(
        current_velocity,
        current_pos,
        target_pos,
        &neighbors,
        speed,
        agility,
        acceleration,
        balance,
        fatigue_multiplier,
        physical_radius,
        dt,
    )
}

pub fn derive_player_dynamic_boid_steered_velocity(
    current_velocity: Velocity,
    current_pos: Position,
    target_pos: Position,
    neighbors: &[SpatialNeighbor],
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_multiplier: f64,
    dt: Duration,
) -> Velocity {
    let speed = calculate_player_speed(player, attribute_keys, fatigue_multiplier);
    let agility = extract_attribute_value(player, attribute_keys, AttributeKey::Agility);
    let acceleration = extract_attribute_value(player, attribute_keys, AttributeKey::Acceleration);
    let balance = extract_attribute_value(player, attribute_keys, AttributeKey::Balance);
    let physical_radius = derive_player_physical_radius(player, attribute_keys);

    calculate_dynamic_boid_steering_velocity(
        current_velocity,
        current_pos,
        target_pos,
        neighbors,
        speed,
        agility,
        acceleration,
        balance,
        fatigue_multiplier,
        physical_radius,
        dt,
    )
}