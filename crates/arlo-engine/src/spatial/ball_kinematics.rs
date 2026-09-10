use crate::attributes::PlayerAttributeTable;
use crate::physical::systems::degradation::extract_effective_attribute_value;
use crate::physical::PhysicalState;
use arlo_domain::sport_constants::{
    BASE_CROSS_SPEED_METERS_PER_SEC, BASE_SHOT_SPEED_METERS_PER_SEC,
    BASE_THROW_SPEED_METERS_PER_SEC, CROSS_SPEED_CROSSING_SCALE, SHOT_SPEED_FINISHING_SCALE,
    SHOT_SPEED_TECHNIQUE_SCALE, THROW_SPEED_PASSING_SCALE, THROW_SPEED_TECHNIQUE_SCALE,
};
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::{Duration, Length, Speed, MIRIM_TO_METERS};
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_pass_speed_from_table(
    _player: &Player,
    table: &PlayerAttributeTable,
    state: &PhysicalState,
) -> Speed {
    let passing = extract_effective_attribute_value(table, AttributeKey::Passing, state);
    let technique = extract_effective_attribute_value(table, AttributeKey::Technique, state);
    let speed_val = BASE_THROW_SPEED_METERS_PER_SEC
        + (passing * THROW_SPEED_PASSING_SCALE)
        + (technique * THROW_SPEED_TECHNIQUE_SCALE);
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
    let speed_val = BASE_CROSS_SPEED_METERS_PER_SEC + (crossing * CROSS_SPEED_CROSSING_SCALE);
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

pub fn calculate_shot_speed_from_table(
    _player: &Player,
    table: &PlayerAttributeTable,
    state: &PhysicalState,
) -> Speed {
    let finishing = extract_effective_attribute_value(table, AttributeKey::Finishing, state);
    let technique = extract_effective_attribute_value(table, AttributeKey::Technique, state);
    let speed_val = BASE_SHOT_SPEED_METERS_PER_SEC
        + (finishing * SHOT_SPEED_FINISHING_SCALE)
        + (technique * SHOT_SPEED_TECHNIQUE_SCALE);
    Speed::new(speed_val)
}

pub fn calculate_shot_speed(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    state: &PhysicalState,
) -> Speed {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    calculate_shot_speed_from_table(player, &table, state)
}

pub fn ball_flight_duration(distance_mirim: f64, ball_speed: Speed) -> Duration {
    let distance = Length::new(distance_mirim * MIRIM_TO_METERS);
    if ball_speed.value() <= 0.0 {
        Duration::new(0.0)
    } else {
        Duration::new(distance.value() / ball_speed.value())
    }
}
