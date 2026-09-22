use crate::scoring_regime::ScoringRegimePolicy;
use crate::world_state::analysis::decision_bias::{calculate_decision_bias, calculate_kick_foul_bias};
use crate::world_state::analysis::deficit_urgency::calculate_urgency_index;
use crate::world_state::analysis::lead_pressure::{calculate_lead_pressure, LeadPressureProfile};
use crate::world_state::analysis::time_urgency::{calculate_time_urgency, calculate_total_remaining_seconds};
use crate::world_state::core::constants::*;
use crate::world_state::match_state::MatchState;
use arlo_domain::{ArtrineDecisionKind, KickFoulDecisionKind, KickFoulScoringTier};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GameStatePressure {
    score_deficit: i32,
    total_remaining_seconds: f64,
    urgency_index: f64,
    offensive_risk_bias: f64,
    turnover_aversion_scale: f64,
}

impl GameStatePressure {
    pub fn new(
        score_deficit: i32,
        total_remaining_seconds: f64,
        urgency_index: f64,
        offensive_risk_bias: f64,
        turnover_aversion_scale: f64,
    ) -> Self {
        Self {
            score_deficit,
            total_remaining_seconds,
            urgency_index,
            offensive_risk_bias,
            turnover_aversion_scale,
        }
    }

    pub fn score_deficit(&self) -> i32 {
        self.score_deficit
    }

    pub fn total_remaining_seconds(&self) -> f64 {
        self.total_remaining_seconds
    }

    pub fn urgency_index(&self) -> f64 {
        self.urgency_index
    }

    pub fn offensive_risk_bias(&self) -> f64 {
        self.offensive_risk_bias
    }

    pub fn turnover_aversion_scale(&self) -> f64 {
        self.turnover_aversion_scale
    }

    pub fn is_trailing(&self) -> bool {
        self.score_deficit > 0
    }

    pub fn is_leading(&self) -> bool {
        self.score_deficit < 0
    }

    pub fn is_tied(&self) -> bool {
        self.score_deficit == 0
    }

    pub fn action_risk_multiplier(&self, risk_coeff: f64) -> f64 {
        (1.0 + risk_coeff * self.offensive_risk_bias).clamp(MIN_DECISION_BIAS, MAX_DECISION_BIAS)
    }

    pub fn goal_point_bias(&self) -> f64 {
        self.action_risk_multiplier(0.80)
    }

    pub fn field_point_bias(&self) -> f64 {
        self.action_risk_multiplier(0.20)
    }

    pub fn long_launch_bias(&self) -> f64 {
        self.action_risk_multiplier(0.70)
    }

    pub fn short_pass_bias(&self) -> f64 {
        self.action_risk_multiplier(-0.40)
    }

    pub fn carry_bias(&self) -> f64 {
        self.action_risk_multiplier(0.10)
    }

    pub fn cross_bias(&self) -> f64 {
        self.action_risk_multiplier(0.65)
    }

    pub fn self_finish_bias(&self) -> f64 {
        self.action_risk_multiplier(0.60)
    }

    pub fn bias_for_decision(
        &self,
        kind: ArtrineDecisionKind,
        drives_in_series: u32,
        is_bonus_phase: bool,
        regime: &ScoringRegimePolicy,
        down: u8,
        normalized_proximity: f64,
    ) -> f64 {
        calculate_decision_bias(kind, drives_in_series, is_bonus_phase, regime, self.offensive_risk_bias, down, normalized_proximity)
    }

    pub fn bias_for_kick_foul_decision(
        &self,
        kind: KickFoulDecisionKind,
        tier: KickFoulScoringTier,
        lateral_ratio: f64,
    ) -> f64 {
        calculate_kick_foul_bias(kind, tier, lateral_ratio, self.offensive_risk_bias)
    }
}

impl Default for GameStatePressure {
    fn default() -> Self {
        Self {
            score_deficit: 0,
            total_remaining_seconds: DEFAULT_TOTAL_REMAINING_SECONDS,
            urgency_index: 0.0,
            offensive_risk_bias: 0.0,
            turnover_aversion_scale: DEFAULT_PRESSURE_BIAS,
        }
    }
}

pub fn analyze_game_state_with_lead_pressure(
    score_offense: u32,
    score_defense: u32,
    period: u32,
    regulation_periods: u32,
    seconds_in_period: f64,
    period_duration_seconds: f64,
    lead_profile: &LeadPressureProfile,
) -> GameStatePressure {
    let score_deficit = (score_defense as i32) - (score_offense as i32);
    let total_remaining_seconds = calculate_total_remaining_seconds(
        period,
        regulation_periods,
        seconds_in_period,
        period_duration_seconds,
    );

    let time_urgency = calculate_time_urgency(total_remaining_seconds);
    let urgency_index = calculate_urgency_index(score_deficit, time_urgency);

    let normalized_deficit = (score_deficit as f64) / 4.0;
    let lead_pressure = calculate_lead_pressure(score_deficit, time_urgency, lead_profile);
    let offensive_risk_bias = (normalized_deficit * time_urgency - lead_pressure).clamp(-1.0, 1.5);
    let turnover_aversion_scale = (1.0 - 0.50 * offensive_risk_bias).clamp(0.35, 2.50);

    GameStatePressure::new(
        score_deficit,
        total_remaining_seconds,
        urgency_index,
        offensive_risk_bias,
        turnover_aversion_scale,
    )
}

pub fn analyze_game_state(
    score_offense: u32,
    score_defense: u32,
    period: u32,
    regulation_periods: u32,
    seconds_in_period: f64,
    period_duration_seconds: f64,
) -> GameStatePressure {
    analyze_game_state_with_lead_pressure(
        score_offense,
        score_defense,
        period,
        regulation_periods,
        seconds_in_period,
        period_duration_seconds,
        &LeadPressureProfile::default(),
    )
}

pub fn analyze_match_state(state: &MatchState) -> GameStatePressure {
    let is_home_offense = state
        .possession()
        .role()
        .is_offense(state.home_team_id());
    let (score_offense, score_defense) = if is_home_offense {
        (
            state.home_score().total_points,
            state.away_score().total_points,
        )
    } else {
        (
            state.away_score().total_points,
            state.home_score().total_points,
        )
    };
    analyze_game_state(
        score_offense,
        score_defense,
        state.clock().period(),
        state.format_rules().regulation_periods(),
        state.clock().seconds_in_period(),
        state.clock().period_duration_seconds(),
    )
}