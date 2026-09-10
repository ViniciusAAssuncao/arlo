use crate::attributes::PlayerAttributeTable;
use crate::physical::models::metabolic_power::{
    calculate_desired_cruise_speed,
    calculate_max_acceleration,
    calculate_player_body_mass_from_table,
    calculate_player_critical_speed_from_table,
};
use crate::physical::state::PhysicalState;
use crate::psychology::state::ImpulseState;
use crate::psychology::systems::baseline::{
    calculate_player_impulse_baseline_from_table_with_profile,
};
use crate::spatial::decision_vector::extract_attribute_value;
use crate::world_state::context_analyzer::GameStatePressure;
use arlo_domain::{ AttributeKey, Player };
use arlo_math::units::{ Position, Speed, MIRIM_TO_METERS };
use serde::{ Deserialize, Serialize };

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
        is_overridden: bool
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

pub struct PacingRequest<'a> {
    pub base_cruise_speed: f64,
    pub critical_speed: f64,
    pub raw_max_acceleration: f64,
    pub work_rate: f64,
    pub determination: f64,
    pub impulse_value: u8,
    pub baseline_impulse: f64,
    pub is_near_ball: bool,
    pub game_state_pressure: &'a GameStatePressure,
    pub fatigue_multiplier: f64,
    pub effort_multiplier: f64,
}

pub fn is_player_near_ball(
    player_pos: Position,
    ball_pos: Position,
    proximity_threshold_mirim: f64
) -> bool {
    let dx = player_pos.raw().0 - ball_pos.raw().0;
    let dy = player_pos.raw().1 - ball_pos.raw().1;
    let dist_meters = (dx * dx + dy * dy).sqrt();
    let dist_mirim = dist_meters / MIRIM_TO_METERS;
    dist_mirim <= proximity_threshold_mirim
}

pub fn calculate_pacing_state(request: &PacingRequest<'_>) -> PacingState {
    let norm_wr = request.work_rate.clamp(0.0, 20.0) / 20.0;
    let norm_det = request.determination.clamp(0.0, 20.0) / 20.0;
    let urgency = request.game_state_pressure.urgency_index().max(0.0);
    let is_trailing = request.game_state_pressure.is_trailing();

    let impulse_delta = (request.impulse_value as f64) - request.baseline_impulse;
    let norm_impulse_delta = (impulse_delta / 50.0).clamp(-1.0, 1.0);
    let mental_drive = 0.55 * norm_wr + 0.45 * norm_det;

    let base_effort = if request.is_near_ball { 1.0 } else { 0.7 + 0.3 * norm_wr };

    let is_conserving = !request.is_near_ball && norm_wr < 0.75;

    let impulse_effort_mod = if norm_impulse_delta >= 0.0 {
        1.0 + 0.12 * norm_impulse_delta * mental_drive
    } else {
        1.0 - 0.15 * -norm_impulse_delta * (1.2 - 0.4 * mental_drive)
    };

    let (urgency_effort, is_overridden) = if is_trailing && urgency > 0.3 {
        let deficit_scale = (request.game_state_pressure.score_deficit().max(1) as f64) * 0.15;
        let conservation_penalty = (1.0 - norm_wr) * 0.55;
        let impulse_urgency_boost = if norm_impulse_delta > 0.0 {
            0.1 * norm_impulse_delta * mental_drive
        } else {
            -0.1 * -norm_impulse_delta
        };
        let mult =
            1.0 + urgency * (0.4 + conservation_penalty + deficit_scale + impulse_urgency_boost);
        (mult, is_conserving)
    } else if urgency > 0.5 {
        let mult = 1.0 + urgency * (0.2 + 0.05 * norm_impulse_delta * mental_drive);
        (mult, false)
    } else {
        (1.0, false)
    };

    let effort_scale = (
        base_effort *
        urgency_effort *
        impulse_effort_mod *
        request.effort_multiplier
    ).clamp(0.5, 1.8);
    let paced_speed_val = (request.base_cruise_speed * effort_scale).clamp(
        0.5,
        request.critical_speed * 1.15
    );
    let target_cruise_speed = Speed::new(paced_speed_val);

    let paced_accel = (
        request.raw_max_acceleration *
        (0.75 + 0.25 * effort_scale) *
        request.fatigue_multiplier.clamp(0.3, 1.0)
    ).clamp(0.8, request.raw_max_acceleration * 1.3);

    PacingState::new(target_cruise_speed, paced_accel, is_conserving, is_overridden)
}

pub fn calculate_player_pacing_state_from_table(
    player: &Player,
    table: &PlayerAttributeTable,
    is_near_ball: bool,
    game_state_pressure: &GameStatePressure,
    state: &PhysicalState,
    impulse_state: &ImpulseState,
    effort_multiplier: f64,
    current_time_unix_seconds: i64
) -> PacingState {
    let work_rate = extract_attribute_value(table, AttributeKey::WorkRate);
    let determination = extract_attribute_value(table, AttributeKey::Determination);
    let positioning = extract_attribute_value(table, AttributeKey::Positioning);
    let accel_attr = extract_attribute_value(table, AttributeKey::Acceleration);
    let agility_attr = extract_attribute_value(table, AttributeKey::Agility);
    let str_attr = extract_attribute_value(table, AttributeKey::Strength);
    let mass = calculate_player_body_mass_from_table(player, table);

    let v_crit = calculate_player_critical_speed_from_table(
        player,
        table,
        current_time_unix_seconds
    ).value();
    let base_cruise = calculate_desired_cruise_speed(v_crit, work_rate, positioning);
    let raw_max_accel = calculate_max_acceleration(accel_attr, agility_attr, str_attr, mass, 1.0);

    let profile = crate::caching::impulse_baseline_profile();
    let baseline = calculate_player_impulse_baseline_from_table_with_profile(table, profile);

    let req = PacingRequest {
        base_cruise_speed: base_cruise,
        critical_speed: v_crit,
        raw_max_acceleration: raw_max_accel,
        work_rate,
        determination,
        impulse_value: impulse_state.value(),
        baseline_impulse: baseline,
        is_near_ball,
        game_state_pressure,
        fatigue_multiplier: state.energy(),
        effort_multiplier,
    };

    calculate_pacing_state(&req)
}