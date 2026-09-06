use crate::physical::models::metabolic_power::{
    calculate_desired_cruise_speed, calculate_max_acceleration,
    calculate_player_body_mass, calculate_player_critical_speed,
};
use crate::physical::state::PhysicalState;
use crate::spatial::decision_vector::extract_attribute_value;
use crate::world_state::context_analyzer::GameStatePressure;
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::{Position, Speed, MIRIM_TO_METERS};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PacingState {
    target_cruise_speed: Speed,
    max_acceleration: f64,
    is_conserving: bool,
    is_overridden: bool,
}

impl PacingState {
    pub fn new(
        target_cruise_speed: Speed,
        max_acceleration: f64,
        is_conserving: bool,
        is_overridden: bool,
    ) -> Self {
        Self {
            target_cruise_speed,
            max_acceleration,
            is_conserving,
            is_overridden,
        }
    }

    pub fn target_cruise_speed(&self) -> Speed {
        self.target_cruise_speed
    }

    pub fn max_acceleration(&self) -> f64 {
        self.max_acceleration
    }

    pub fn is_conserving(&self) -> bool {
        self.is_conserving
    }

    pub fn is_overridden(&self) -> bool {
        self.is_overridden
    }
}

impl Default for PacingState {
    fn default() -> Self {
        Self {
            target_cruise_speed: Speed::new(2.5),
            max_acceleration: 3.5,
            is_conserving: false,
            is_overridden: false,
        }
    }
}

pub fn is_player_near_ball(
    player_pos: Position,
    ball_pos: Position,
    proximity_threshold_mirim: f64,
) -> bool {
    let dx = player_pos.raw().0 - ball_pos.raw().0;
    let dy = player_pos.raw().1 - ball_pos.raw().1;
    let dist_meters = (dx * dx + dy * dy).sqrt();
    let dist_mirim = dist_meters / MIRIM_TO_METERS;
    dist_mirim <= proximity_threshold_mirim
}

pub fn calculate_pacing_state(
    base_cruise_speed: f64,
    critical_speed: f64,
    raw_max_acceleration: f64,
    work_rate: f64,
    is_near_ball: bool,
    game_state_pressure: &GameStatePressure,
    fatigue_multiplier: f64,
) -> PacingState {
    let norm_wr = (work_rate.clamp(0.0, 20.0)) / 20.0;
    let urgency = game_state_pressure.urgency_index().max(0.0);
    let is_trailing = game_state_pressure.is_trailing();

    let base_effort = if is_near_ball {
        1.0
    } else {
        0.70 + 0.30 * norm_wr
    };

    let is_conserving = !is_near_ball && norm_wr < 0.75;

    let (urgency_effort, is_overridden) = if is_trailing && urgency > 0.30 {
        let deficit_scale = (game_state_pressure.score_deficit().max(1) as f64) * 0.15;
        let conservation_penalty = (1.0 - norm_wr) * 0.55;
        let mult = 1.0 + (urgency * (0.40 + conservation_penalty + deficit_scale));
        (mult, is_conserving)
    } else if urgency > 0.50 {
        (1.0 + urgency * 0.20, false)
    } else {
        (1.0, false)
    };

    let effort_scale = (base_effort * urgency_effort).clamp(0.50, 1.80);
    let paced_speed_val = (base_cruise_speed * effort_scale).clamp(0.5, critical_speed * 1.15);
    let target_cruise_speed = Speed::new(paced_speed_val);

    let paced_accel = (raw_max_acceleration * (0.75 + 0.25 * effort_scale) * fatigue_multiplier.clamp(0.3, 1.0)).clamp(0.8, raw_max_acceleration * 1.3);

    PacingState::new(target_cruise_speed, paced_accel, is_conserving, is_overridden)
}

pub fn calculate_player_pacing_state(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    is_near_ball: bool,
    game_state_pressure: &GameStatePressure,
    state: &PhysicalState,
    current_time_unix_seconds: i64,
) -> PacingState {
    let work_rate = extract_attribute_value(player, attribute_keys, AttributeKey::WorkRate);
    let positioning = extract_attribute_value(player, attribute_keys, AttributeKey::Positioning);
    let accel_attr = extract_attribute_value(player, attribute_keys, AttributeKey::Acceleration);
    let agility_attr = extract_attribute_value(player, attribute_keys, AttributeKey::Agility);
    let str_attr = extract_attribute_value(player, attribute_keys, AttributeKey::Strength);
    let mass = calculate_player_body_mass(player, attribute_keys);

    let v_crit = calculate_player_critical_speed(player, attribute_keys, current_time_unix_seconds).value();
    let base_cruise = calculate_desired_cruise_speed(v_crit, work_rate, positioning);
    let raw_max_accel = calculate_max_acceleration(accel_attr, agility_attr, str_attr, mass, 1.0);

    calculate_pacing_state(
        base_cruise,
        v_crit,
        raw_max_accel,
        work_rate,
        is_near_ball,
        game_state_pressure,
        state.energy(),
    )
}

pub fn calculate_paced_distance_mirim(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    duration_seconds: f64,
    is_near_ball: bool,
    game_state_pressure: &GameStatePressure,
    state: &PhysicalState,
    current_time_unix_seconds: i64,
) -> f64 {
    let pacing = calculate_player_pacing_state(
        player,
        attribute_keys,
        is_near_ball,
        game_state_pressure,
        state,
        current_time_unix_seconds,
    );
    let dist_meters = pacing.target_cruise_speed().value() * duration_seconds.max(0.0);
    dist_meters / MIRIM_TO_METERS
}