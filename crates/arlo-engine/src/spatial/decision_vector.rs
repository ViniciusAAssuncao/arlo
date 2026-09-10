use crate::attributes::PlayerAttributeTable;
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::physical::PhysicalState;
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::{Position, Speed, Velocity};
use std::collections::HashMap;
use uuid::Uuid;

pub fn extract_attribute_value(table: &PlayerAttributeTable, target: AttributeKey) -> f64 {
    table.get(target)
}

pub fn extract_attribute_value_from_player(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    target: AttributeKey,
) -> f64 {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    table.get(target)
}

pub fn calculate_player_speed(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_multiplier: f64,
) -> Speed {
    let state = PhysicalState::with_energy(fatigue_multiplier);
    calculate_effective_player_speed(player, attribute_keys, &state)
}

pub fn derive_velocity_towards_target(
    current_pos: Position,
    target_pos: Position,
    speed: Speed,
) -> Velocity {
    let delta = target_pos.raw() - current_pos.raw();
    let dist = delta.magnitude();
    if dist < 1e-6 {
        Velocity::zero()
    } else {
        let dir = delta / dist;
        let raw_vel = dir * speed.value();
        Velocity::from_raw(raw_vel)
    }
}

pub fn derive_player_velocity_towards_target(
    current_pos: Position,
    target_pos: Position,
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_multiplier: f64,
) -> Velocity {
    let speed = calculate_player_speed(player, attribute_keys, fatigue_multiplier);
    derive_velocity_towards_target(current_pos, target_pos, speed)
}
