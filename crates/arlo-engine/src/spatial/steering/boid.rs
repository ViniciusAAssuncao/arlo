use crate::physical::models::metabolic_power::calculate_max_acceleration;
use crate::spatial::movement_context::MovementContext;
use crate::spatial::steering::forces::{
    calculate_dead_ball_separation_force_with_id, calculate_dynamic_separation_force_with_id,
    calculate_seek_force,
};
use crate::spatial::steering::neighbor::SpatialNeighbor;
use crate::spatial::steering::radii::{derive_arrival_slowing_radius, max_turn_radians_per_tick};
use arlo_math::units::{Duration, Position, Speed, Velocity};
use std::f64::consts::PI;
use uuid::Uuid;

pub fn calculate_dynamic_boid_steering_velocity_with_context(
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
    strength: f64,
    mass_kg: f64,
    fatigue_multiplier: f64,
    self_physical_radius: f64,
    context: MovementContext,
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
        mass_kg,
        dt,
    );

    let raw_separation = match context {
        MovementContext::DeadBall => calculate_dead_ball_separation_force_with_id(
            current_pos,
            current_velocity,
            self_physical_radius,
            self_id,
            neighbors,
        ),
        MovementContext::LivePlay => calculate_dynamic_separation_force_with_id(
            current_pos,
            current_velocity,
            self_physical_radius,
            self_id,
            self_is_home,
            neighbors,
        ),
    };

    let dist_to_target = (target_pos.raw() - current_pos.raw()).magnitude();
    let sep_scale = if arrival_radius > 1e-4 {
        (dist_to_target / arrival_radius).clamp(0.0, 1.0)
    } else {
        1.0
    };

    let raw_accel =
        calculate_max_acceleration(acceleration, agility, strength, mass_kg, fatigue_multiplier);

    let max_accel = match context {
        MovementContext::DeadBall => (raw_accel * 0.55).clamp(1.0, 3.5),
        MovementContext::LivePlay => raw_accel,
    };

    let max_muscular_force = mass_kg * max_accel;

    let separation_factor = match context {
        MovementContext::DeadBall => 0.25,
        MovementContext::LivePlay => 0.45,
    };

    let separation_force = raw_separation * (max_muscular_force * separation_factor * sep_scale);
    let combined_force = seek_force + separation_force;
    let force_magnitude = combined_force.magnitude();

    let clamped_force = if force_magnitude > max_muscular_force && force_magnitude > 1e-6 {
        (combined_force / force_magnitude) * max_muscular_force
    } else {
        combined_force
    };

    let applied_accel = clamped_force / mass_kg.max(1.0);
    let candidate_raw = current_velocity.raw() + (applied_accel * dt.value());
    let candidate_speed = candidate_raw.magnitude();

    if candidate_speed < 1e-6 {
        return Velocity::zero();
    }

    let effective_max_speed = speed.value();
    let speed_capped_raw = if candidate_speed > effective_max_speed {
        (candidate_raw / candidate_speed) * effective_max_speed
    } else {
        candidate_raw
    };

    if current_speed < 1e-6 {
        return Velocity::from_raw(speed_capped_raw);
    }

    let current_dir = current_velocity.raw() / current_speed;
    let candidate_dir = speed_capped_raw / speed_capped_raw.magnitude();

    let current_angle = current_dir.1.atan2(current_dir.0);
    let target_angle = candidate_dir.1.atan2(candidate_dir.0);

    let mut diff = target_angle - current_angle;
    while diff > PI {
        diff -= 2.0 * PI;
    }
    while diff < -PI {
        diff += 2.0 * PI;
    }

    let base_turn = max_turn_radians_per_tick(agility);
    let max_turn = match context {
        MovementContext::DeadBall => (base_turn * 0.85).max(0.18),
        MovementContext::LivePlay => base_turn,
    };

    let applied_diff = diff.clamp(-max_turn, max_turn);
    let new_angle = current_angle + applied_diff;

    let final_dir = Position::from_components(new_angle.cos(), new_angle.sin(), 0.0).raw();
    let final_raw = final_dir * speed_capped_raw.magnitude();
    Velocity::from_raw(final_raw)
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
    strength: f64,
    mass_kg: f64,
    fatigue_multiplier: f64,
    self_physical_radius: f64,
    dt: Duration,
) -> Velocity {
    calculate_dynamic_boid_steering_velocity_with_context(
        current_velocity,
        current_pos,
        target_pos,
        self_id,
        self_is_home,
        neighbors,
        speed,
        agility,
        acceleration,
        balance,
        strength,
        mass_kg,
        fatigue_multiplier,
        self_physical_radius,
        MovementContext::LivePlay,
        dt,
    )
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
    strength: f64,
    mass_kg: f64,
    fatigue_multiplier: f64,
    self_physical_radius: f64,
    dt: Duration,
) -> Velocity {
    calculate_dynamic_boid_steering_velocity_with_context(
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
        strength,
        mass_kg,
        fatigue_multiplier,
        self_physical_radius,
        MovementContext::LivePlay,
        dt,
    )
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
        10.0,
        78.0,
        1.0,
        0.55,
        dt,
    )
}
