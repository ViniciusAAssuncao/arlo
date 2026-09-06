use crate::spatial::decision_vector::extract_attribute_value;
use crate::world_state::context_analyzer::GameStatePressure;
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::{Position, MIRIM_TO_METERS};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PacingState {
    pacing_multiplier: f64,
    is_conserving: bool,
    is_overridden: bool,
}

impl PacingState {
    pub fn new(pacing_multiplier: f64, is_conserving: bool, is_overridden: bool) -> Self {
        Self {
            pacing_multiplier,
            is_conserving,
            is_overridden,
        }
    }

    pub fn pacing_multiplier(&self) -> f64 {
        self.pacing_multiplier
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
            pacing_multiplier: 1.0,
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
    work_rate: f64,
    is_near_ball: bool,
    game_state_pressure: &GameStatePressure,
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

    let (urgency_multiplier, is_overridden) = if is_trailing && urgency > 0.30 {
        let deficit_scale = (game_state_pressure.score_deficit().max(1) as f64) * 0.15;
        let conservation_penalty = (1.0 - norm_wr) * 0.55;
        let mult = 1.0 + (urgency * (0.40 + conservation_penalty + deficit_scale));
        (mult, is_conserving)
    } else if urgency > 0.50 {
        (1.0 + urgency * 0.20, false)
    } else {
        (1.0, false)
    };

    let pacing_multiplier = (base_effort * urgency_multiplier).clamp(0.50, 1.80);

    PacingState::new(pacing_multiplier, is_conserving, is_overridden)
}

pub fn calculate_pacing_multiplier(
    work_rate: f64,
    is_near_ball: bool,
    game_state_pressure: &GameStatePressure,
) -> f64 {
    calculate_pacing_state(work_rate, is_near_ball, game_state_pressure).pacing_multiplier()
}

pub fn calculate_player_pacing_state(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    is_near_ball: bool,
    game_state_pressure: &GameStatePressure,
) -> PacingState {
    let work_rate = extract_attribute_value(player, attribute_keys, AttributeKey::WorkRate);
    calculate_pacing_state(work_rate, is_near_ball, game_state_pressure)
}

pub fn calculate_player_pacing_multiplier(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    is_near_ball: bool,
    game_state_pressure: &GameStatePressure,
) -> f64 {
    calculate_player_pacing_state(player, attribute_keys, is_near_ball, game_state_pressure)
        .pacing_multiplier()
}

pub fn calculate_paced_distance_mirim(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    raw_distance_mirim: f64,
    is_near_ball: bool,
    game_state_pressure: &GameStatePressure,
) -> f64 {
    let mult = calculate_player_pacing_multiplier(
        player,
        attribute_keys,
        is_near_ball,
        game_state_pressure,
    );
    raw_distance_mirim.max(0.0) * mult
}