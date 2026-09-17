use crate::attributes::PlayerAttributeTable;
use crate::physical::systems::degradation::extract_effective_attribute_value;
use crate::physical::PhysicalState;
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::{Duration, Position as VectorPosition, Speed, MIRIM_TO_METERS};
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_distance_mirim(a: VectorPosition, b: VectorPosition) -> f64 {
    let dx = a.raw().0 - b.raw().0;
    let dy = a.raw().1 - b.raw().1;
    let dz = a.raw().2 - b.raw().2;
    (dx * dx + dy * dy + dz * dz).sqrt() / MIRIM_TO_METERS
}

pub fn calculate_pass_speed_from_table(
    _player: &Player,
    table: &PlayerAttributeTable,
    state: &PhysicalState,
) -> Speed {
    let passing = extract_effective_attribute_value(table, AttributeKey::Passing, state);
    let technique = extract_effective_attribute_value(table, AttributeKey::Technique, state);
    let speed_val = 14.0 + (passing * 0.45) + (technique * 0.15);
    Speed::new(speed_val)
}

pub fn calculate_pass_speed(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    state: &PhysicalState,
) -> Speed {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    calculate_pass_speed_from_table(player, &table, state)
}

pub fn calculate_cross_speed_from_table(
    _player: &Player,
    table: &PlayerAttributeTable,
    state: &PhysicalState,
) -> Speed {
    let crossing = extract_effective_attribute_value(table, AttributeKey::Crossing, state);
    let technique = extract_effective_attribute_value(table, AttributeKey::Technique, state);
    let speed_val = 15.0 + (crossing * 0.45) + (technique * 0.15);
    Speed::new(speed_val)
}

pub fn calculate_cross_speed(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    state: &PhysicalState,
) -> Speed {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    calculate_cross_speed_from_table(player, &table, state)
}

pub fn ball_flight_duration(distance_mirim: f64, speed: Speed) -> Duration {
    let distance_meters = distance_mirim * MIRIM_TO_METERS;
    let seconds = (distance_meters / speed.value().max(1.0)).max(0.05);
    Duration::new(seconds)
}