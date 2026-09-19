use crate::world_state::constants::*;
use crate::world_state::MatchState;
use arlo_domain::sport_constants::{
    KICK_FOUL_CROSS_LATERAL_BIAS_BASE, KICK_FOUL_CROSS_LATERAL_BIAS_SCALE,
    KICK_FOUL_SHOOT_GOAL_POINT_BIAS_WEIGHT_FIRST_ZONE,
    KICK_FOUL_SHOOT_GOAL_POINT_BIAS_WEIGHT_STANDARD,
};
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

    pub fn score_deficit(&self) -> i32 { self.score_deficit }
    pub fn total_remaining_seconds(&self) -> f64 { self.total_remaining_seconds }
    pub fn urgency_index(&self) -> f64 { self.urgency_index }
    pub fn offensive_risk_bias(&self) -> f64 { self.offensive_risk_bias }
    pub fn turnover_aversion_scale(&self) -> f64 { self.turnover_aversion_scale }
    pub fn is_trailing(&self) -> bool { self.score_deficit > 0 }
    pub fn is_leading(&self) -> bool { self.score_deficit < 0 }
    pub fn is_tied(&self) -> bool { self.score_deficit == 0 }

    fn action_risk_multiplier(&self, risk_coeff: f64) -> f64 {
        (1.0 + risk_coeff * self.offensive_risk_bias).clamp(MIN_DECISION_BIAS, MAX_DECISION_BIAS)
    }

    pub fn goal_point_bias(&self) -> f64 { self.action_risk_multiplier(0.60) }
    pub fn field_point_bias(&self) -> f64 { self.action_risk_multiplier(0.35) }
    pub fn long_launch_bias(&self) -> f64 { self.action_risk_multiplier(0.70) }
    pub fn short_pass_bias(&self) -> f64 { self.action_risk_multiplier(-0.40) }
    pub fn carry_bias(&self) -> f64 { self.action_risk_multiplier(-0.35) }
    pub fn cross_bias(&self) -> f64 { self.action_risk_multiplier(0.65) }
    pub fn self_finish_bias(&self) -> f64 { self.action_risk_multiplier(0.55) }

    pub fn bias_for_decision(&self, kind: ArtrineDecisionKind, drives_in_series: u32) -> f64 {
        let raw = match kind {
            ArtrineDecisionKind::SelfCarry => {
                let base = self.carry_bias();
                if drives_in_series < DRIVES_THRESHOLD_FOR_SCORING_OPPORTUNITY {
                    base * (1.0 + CARRY_EARLY_DRIVE_BONUS_MULTIPLIER * ((DRIVES_THRESHOLD_FOR_SCORING_OPPORTUNITY - drives_in_series) as f64))
                } else {
                    base
                }
            }
            ArtrineDecisionKind::ShortPass => self.short_pass_bias(),
            ArtrineDecisionKind::LongLaunch => self.long_launch_bias(),
            ArtrineDecisionKind::Cross => {
                let scoring = if drives_in_series >= DRIVES_THRESHOLD_FOR_SCORING_OPPORTUNITY { self.goal_point_bias() } else { self.field_point_bias() };
                self.cross_bias() * scoring
            }
            ArtrineDecisionKind::SelfFinish => {
                let scoring = if drives_in_series >= DRIVES_THRESHOLD_FOR_SCORING_OPPORTUNITY { self.goal_point_bias() } else { self.field_point_bias() };
                self.self_finish_bias() * scoring
            }
        };
        raw.clamp(MIN_DECISION_BIAS, MAX_DECISION_BIAS)
    }

    pub fn bias_for_kick_foul_decision(&self, kind: KickFoulDecisionKind, tier: KickFoulScoringTier, lateral_ratio: f64) -> f64 {
        let raw = match kind {
            KickFoulDecisionKind::Shoot => {
                let weight = match tier {
                    KickFoulScoringTier::FirstZone => KICK_FOUL_SHOOT_GOAL_POINT_BIAS_WEIGHT_FIRST_ZONE,
                    KickFoulScoringTier::Standard => KICK_FOUL_SHOOT_GOAL_POINT_BIAS_WEIGHT_STANDARD,
                };
                self.self_finish_bias() * (1.0 - weight) + self.goal_point_bias() * weight
            }
            KickFoulDecisionKind::Cross => {
                let lateral_factor = KICK_FOUL_CROSS_LATERAL_BIAS_BASE + KICK_FOUL_CROSS_LATERAL_BIAS_SCALE * lateral_ratio.clamp(0.0, 1.0);
                self.cross_bias() * lateral_factor
            }
            KickFoulDecisionKind::ShortPass => self.short_pass_bias(),
            KickFoulDecisionKind::LongLaunch => self.long_launch_bias(),
        };
        raw.clamp(MIN_DECISION_BIAS, MAX_DECISION_BIAS)
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

pub fn analyze_game_state(
    score_offense: u32,
    score_defense: u32,
    period: u32,
    regulation_periods: u32,
    seconds_in_period: f64,
    period_duration_seconds: f64,
) -> GameStatePressure {
    let score_deficit = (score_defense as i32) - (score_offense as i32);
    let rem_in_period = (period_duration_seconds - seconds_in_period).max(0.0);
    let total_remaining_seconds = if period <= regulation_periods {
        ((regulation_periods - period) as f64) * period_duration_seconds + rem_in_period
    } else {
        rem_in_period
    };

    let time_urgency = 1.0 / (1.0 + (total_remaining_seconds / URGENCY_TIME_HALF_LIFE_SECONDS).powf(URGENCY_POWER_CURVE));
    let urgency_index = if score_deficit > 0 {
        ((score_deficit as f64) * TRAILING_URGENCY_DEFICIT_FACTOR * (TRAILING_URGENCY_BASE_WEIGHT + TRAILING_URGENCY_TIME_WEIGHT * time_urgency)).clamp(0.0, MAX_TRAILING_URGENCY_INDEX)
    } else if score_deficit < 0 {
        ((-score_deficit as f64) * LEADING_URGENCY_DEFICIT_FACTOR * time_urgency).clamp(0.0, MAX_LEADING_URGENCY_INDEX)
    } else {
        (TIED_URGENCY_BASE_FACTOR * time_urgency).clamp(0.0, MAX_TIED_URGENCY_INDEX)
    };

    let normalized_deficit = (score_deficit as f64) / 4.0;
    let offensive_risk_bias = (normalized_deficit * time_urgency).clamp(-1.0, 1.5);
    let turnover_aversion_scale = (1.0 - 0.50 * offensive_risk_bias).clamp(0.35, 2.50);

    GameStatePressure::new(score_deficit, total_remaining_seconds, urgency_index, offensive_risk_bias, turnover_aversion_scale)
}

pub fn analyze_match_state(state: &MatchState) -> GameStatePressure {
    let is_home_offense = state.possession().role().is_offense(state.home_team_id());
    let (score_offense, score_defense) = if is_home_offense {
        (state.home_score().total_points, state.away_score().total_points)
    } else {
        (state.away_score().total_points, state.home_score().total_points)
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