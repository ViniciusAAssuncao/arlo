use crate::spatial::steering::neighbor::SpatialNeighbor;
use crate::spatial::steering::radii::{derive_dynamic_separation_radius, max_turn_radians_per_tick};
use arlo_math::units::{Duration, Position, Speed, Vector3, Velocity};
use std::f64::consts::PI;
use uuid::Uuid;

pub fn calculate_seek_force(
    current_pos: Position,
    current_velocity: Velocity,
    target_pos: Position,
    desired_speed: Speed,
    arrival_radius_meters: f64,
    mass_kg: f64,
    dt: Duration,
) -> Vector3 {
    let delta = target_pos.raw() - current_pos.raw();
    let dist = delta.magnitude();
    if dist < 1e-6 {
        return (-current_velocity.raw() * mass_kg) / dt.value().max(1e-4);
    }

    let target_speed = if dist < arrival_radius_meters && arrival_radius_meters > 1e-6 {
        desired_speed.value() * (dist / arrival_radius_meters)
    } else {
        desired_speed.value()
    };

    let target_velocity = (delta / dist) * target_speed;
    let accel = (target_velocity - current_velocity.raw()) / dt.value().max(1e-4);
    accel * mass_kg
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

pub fn calculate_dead_ball_separation_force_with_id(
    current_pos: Position,
    _current_velocity: Velocity,
    self_physical_radius: f64,
    self_id: Uuid,
    neighbors: &[SpatialNeighbor],
) -> Vector3 {
    let mut total_repulsion = Vector3::zero();
    let p0 = current_pos.raw();

    for neighbor in neighbors {
        if neighbor.id == self_id && !self_id.is_nil() {
            continue;
        }
        let pn = neighbor.position.raw();
        let diff = p0 - pn;
        let dist = diff.magnitude();

        if dist > 1e-4 {
            let combined_radius = self_physical_radius + neighbor.physical_radius;
            let sep_radius = combined_radius * 1.50;
            if dist < sep_radius {
                let normalized_dist = dist / sep_radius;
                let weight = (1.0 - normalized_dist) / dist;
                total_repulsion = total_repulsion + (diff * weight);
            }
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
    Velocity::from_raw(final_raw_vel(raw_vel))
}

fn final_raw_vel(vel: arlo_math::units::Vector3) -> arlo_math::units::Vector3 {
    vel
}