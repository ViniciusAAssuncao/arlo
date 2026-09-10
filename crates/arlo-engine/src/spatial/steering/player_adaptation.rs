use crate::attributes::PlayerAttributeTable;
use crate::physical::models::metabolic_power::estimate_body_mass;
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::physical::PhysicalState;
use crate::spatial::decision_vector::extract_attribute_value;
use crate::spatial::steering::boid::calculate_dynamic_boid_steering_velocity;
use crate::spatial::steering::forces::calculate_steered_velocity;
use crate::spatial::steering::neighbor::SpatialNeighbor;
use crate::spatial::steering::radii::derive_player_physical_radius;
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::{Duration, Position, Velocity};
use std::collections::HashMap;
use uuid::Uuid;

pub fn derive_player_steered_velocity(
    current_velocity: Velocity,
    current_pos: Position,
    target_pos: Position,
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_multiplier: f64,
) -> Velocity {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    let state = PhysicalState::with_energy(fatigue_multiplier);
    let speed = calculate_effective_player_speed(player, attribute_keys, &state);
    let agility = extract_attribute_value(&table, AttributeKey::Agility);
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
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    let state = PhysicalState::with_energy(fatigue_multiplier);
    let speed = calculate_effective_player_speed(player, attribute_keys, &state);
    let agility = extract_attribute_value(&table, AttributeKey::Agility);
    let acceleration = extract_attribute_value(&table, AttributeKey::Acceleration);
    let balance = extract_attribute_value(&table, AttributeKey::Balance);
    let strength = extract_attribute_value(&table, AttributeKey::Strength);
    let mass = estimate_body_mass(player.height_m(), strength);
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
        strength,
        mass,
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
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    let state = PhysicalState::with_energy(fatigue_multiplier);
    let speed = calculate_effective_player_speed(player, attribute_keys, &state);
    let agility = extract_attribute_value(&table, AttributeKey::Agility);
    let acceleration = extract_attribute_value(&table, AttributeKey::Acceleration);
    let balance = extract_attribute_value(&table, AttributeKey::Balance);
    let strength = extract_attribute_value(&table, AttributeKey::Strength);
    let mass = estimate_body_mass(player.height_m(), strength);
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
        strength,
        mass,
        fatigue_multiplier,
        physical_radius,
        dt,
    )
}