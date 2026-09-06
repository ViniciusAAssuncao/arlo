use arlo_domain::{AttributeKey, Player};
use arlo_math::units::{Position, Speed, Velocity};
use std::collections::HashMap;
use uuid::Uuid;

pub const BASE_SPRINT_SPEED_METERS_PER_SEC: f64 = 4.0;
pub const PACE_SPEED_SCALE: f64 = 0.25;
pub const ACCELERATION_SPEED_SCALE: f64 = 0.10;

pub fn extract_attribute_value(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    target: AttributeKey,
) -> f64 {
    for attr in player.attributes() {
        if attribute_keys.get(&attr.attribute_definition_id()) == Some(&target) {
            return attr.value() as f64;
        }
    }
    10.0
}

pub fn calculate_player_speed(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_multiplier: f64,
) -> Speed {
    let pace = extract_attribute_value(player, attribute_keys, AttributeKey::Pace);
    let accel = extract_attribute_value(player, attribute_keys, AttributeKey::Acceleration);
    let speed_val = (BASE_SPRINT_SPEED_METERS_PER_SEC
        + (pace * PACE_SPEED_SCALE)
        + (accel * ACCELERATION_SPEED_SCALE))
        * fatigue_multiplier;
    Speed::new(speed_val)
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
