use crate::attributes::PlayerAttributeTable;
use crate::physical::PhysicalState;
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::{Duration, Position as VectorPosition, Speed, MIRIM_TO_METERS};
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_distance_mirim(a: VectorPosition, b: VectorPosition) -> f64 {
    let dx = a.raw().0 - b.raw().0;
    let dy = a.raw().1 - b.raw().1;
    (dx * dx + dy * dy).sqrt() / MIRIM_TO_METERS
}

pub fn calculate_pass_speed_from_table(
    _player: &Player,
    _table: &PlayerAttributeTable,
    _state: &PhysicalState,
) -> Speed {
    Speed::new(18.0)
}

pub fn calculate_pass_speed(
    _player: &Player,
    _attribute_keys: &HashMap<Uuid, AttributeKey>,
    _state: &PhysicalState,
) -> Speed {
    Speed::new(18.0)
}

pub fn calculate_cross_speed_from_table(
    _player: &Player,
    _table: &PlayerAttributeTable,
    _state: &PhysicalState,
) -> Speed {
    Speed::new(16.0)
}

pub fn calculate_cross_speed(
    _player: &Player,
    _attribute_keys: &HashMap<Uuid, AttributeKey>,
    _state: &PhysicalState,
) -> Speed {
    Speed::new(16.0)
}

pub fn ball_flight_duration(distance_mirim: f64, _speed: Speed) -> Duration {
    let seconds = if distance_mirim > 15.0 {
        3.0
    } else if distance_mirim > 6.0 {
        2.0
    } else {
        1.0
    };
    Duration::new(seconds)
}
