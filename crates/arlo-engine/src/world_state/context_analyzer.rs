use crate::world_state::MatchState;
use arlo_domain::ArtrineDecisionKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GameStatePressure {
    score_deficit: i32,
    total_remaining_seconds: f64,
    urgency_index: f64,
    goal_point_bias: f64,
    field_point_bias: f64,
    turnover_aversion_scale: f64,
    long_launch_bias: f64,
    short_pass_bias: f64,
    carry_bias: f64,
    cross_bias: f64,
    self_finish_bias: f64,
}

impl GameStatePressure {
    pub fn new(
        score_deficit: i32,
        total_remaining_seconds: f64,
        urgency_index: f64,
        goal_point_bias: f64,
        field_point_bias: f64,
        turnover_aversion_scale: f64,
        long_launch_bias: f64,
        short_pass_bias: f64,
        carry_bias: f64,
        cross_bias: f64,
        self_finish_bias: f64,
    ) -> Self {
        Self {
            score_deficit,
            total_remaining_seconds,
            urgency_index,
            goal_point_bias,
            field_point_bias,
            turnover_aversion_scale,
            long_launch_bias,
            short_pass_bias,
            carry_bias,
            cross_bias,
            self_finish_bias,
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

    pub fn goal_point_bias(&self) -> f64 {
        self.goal_point_bias
    }

    pub fn field_point_bias(&self) -> f64 {
        self.field_point_bias
    }

    pub fn turnover_aversion_scale(&self) -> f64 {
        self.turnover_aversion_scale
    }

    pub fn long_launch_bias(&self) -> f64 {
        self.long_launch_bias
    }

    pub fn short_pass_bias(&self) -> f64 {
        self.short_pass_bias
    }

    pub fn carry_bias(&self) -> f64 {
        self.carry_bias
    }

    pub fn cross_bias(&self) -> f64 {
        self.cross_bias
    }

    pub fn self_finish_bias(&self) -> f64 {
        self.self_finish_bias
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

    pub fn bias_for_decision(&self, kind: ArtrineDecisionKind, drives_in_series: u32) -> f64 {
        let raw = match kind {
            ArtrineDecisionKind::SelfCarry => self.carry_bias,
            ArtrineDecisionKind::ShortPass => self.short_pass_bias,
            ArtrineDecisionKind::LongLaunch => self.long_launch_bias,
            ArtrineDecisionKind::Cross => {
                if drives_in_series >= 3 {
                    self.cross_bias * self.goal_point_bias
                } else {
                    self.cross_bias * self.field_point_bias
                }
            }
            ArtrineDecisionKind::SelfFinish => {
                if drives_in_series >= 3 {
                    self.self_finish_bias * self.goal_point_bias
                } else {
                    self.self_finish_bias * self.field_point_bias
                }
            }
        };
        raw.clamp(0.10, 4.00)
    }
}

impl Default for GameStatePressure {
    fn default() -> Self {
        Self {
            score_deficit: 0,
            total_remaining_seconds: 3600.0,
            urgency_index: 0.0,
            goal_point_bias: 1.0,
            field_point_bias: 1.0,
            turnover_aversion_scale: 1.0,
            long_launch_bias: 1.0,
            short_pass_bias: 1.0,
            carry_bias: 1.0,
            cross_bias: 1.0,
            self_finish_bias: 1.0,
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

    let time_urgency = 1.0 / (1.0 + (total_remaining_seconds / 360.0).powf(1.4));

    let urgency_index = if score_deficit > 0 {
        ((score_deficit as f64) * 0.35 * (0.30 + 0.70 * time_urgency)).clamp(0.0, 5.0)
    } else if score_deficit < 0 {
        ((-score_deficit as f64) * 0.20 * time_urgency).clamp(0.0, 3.0)
    } else {
        (0.50 * time_urgency).clamp(0.0, 2.0)
    };

    if score_deficit >= 4 && time_urgency > 0.35 {
        let intensity = ((time_urgency - 0.35) / 0.65).clamp(0.0, 1.0);
        let goal_point_bias = (1.0 + 2.20 * intensity).clamp(1.0, 3.50);
        let field_point_bias = (1.0 - 0.75 * intensity).clamp(0.15, 1.00);
        let long_launch_bias = (1.0 + 0.70 * intensity).clamp(1.0, 2.20);
        let cross_bias = (1.0 + 0.90 * intensity).clamp(1.0, 2.40);
        let self_finish_bias = (1.0 + 0.85 * intensity).clamp(1.0, 2.20);
        let short_pass_bias = (1.0 - 0.35 * intensity).clamp(0.45, 1.00);
        let carry_bias = (1.0 - 0.25 * intensity).clamp(0.55, 1.00);
        let turnover_aversion_scale = (1.0 - 0.45 * intensity).clamp(0.35, 1.00);

        GameStatePressure::new(
            score_deficit,
            total_remaining_seconds,
            urgency_index,
            goal_point_bias,
            field_point_bias,
            turnover_aversion_scale,
            long_launch_bias,
            short_pass_bias,
            carry_bias,
            cross_bias,
            self_finish_bias,
        )
    } else if score_deficit > 0 && score_deficit <= 3 && time_urgency > 0.35 {
        let intensity = ((time_urgency - 0.35) / 0.65).clamp(0.0, 1.0);
        let goal_point_bias = (1.0 + 0.80 * intensity).clamp(1.0, 2.00);
        let field_point_bias = (1.0 + 1.40 * intensity).clamp(1.0, 2.60);
        let long_launch_bias = (1.0 + 0.20 * intensity).clamp(1.0, 1.40);
        let cross_bias = (1.0 + 0.70 * intensity).clamp(1.0, 2.00);
        let self_finish_bias = (1.0 + 0.60 * intensity).clamp(1.0, 1.90);
        let short_pass_bias = (1.0 + 0.15 * intensity).clamp(1.0, 1.30);
        let carry_bias = (1.0 + 0.10 * intensity).clamp(1.0, 1.25);
        let turnover_aversion_scale = (1.0 + 0.35 * intensity).clamp(1.0, 1.80);

        GameStatePressure::new(
            score_deficit,
            total_remaining_seconds,
            urgency_index,
            goal_point_bias,
            field_point_bias,
            turnover_aversion_scale,
            long_launch_bias,
            short_pass_bias,
            carry_bias,
            cross_bias,
            self_finish_bias,
        )
    } else if score_deficit < 0 && time_urgency > 0.35 {
        let intensity = ((time_urgency - 0.35) / 0.65).clamp(0.0, 1.0);
        let goal_point_bias = (1.0 - 0.35 * intensity).clamp(0.45, 1.00);
        let field_point_bias = 1.0;
        let long_launch_bias = (1.0 - 0.55 * intensity).clamp(0.35, 1.00);
        let cross_bias = (1.0 - 0.45 * intensity).clamp(0.40, 1.00);
        let self_finish_bias = (1.0 - 0.35 * intensity).clamp(0.50, 1.00);
        let short_pass_bias = (1.0 + 0.45 * intensity).clamp(1.0, 1.70);
        let carry_bias = (1.0 + 0.55 * intensity).clamp(1.0, 1.85);
        let turnover_aversion_scale = (1.0 + 1.10 * intensity).clamp(1.0, 2.60);

        GameStatePressure::new(
            score_deficit,
            total_remaining_seconds,
            urgency_index,
            goal_point_bias,
            field_point_bias,
            turnover_aversion_scale,
            long_launch_bias,
            short_pass_bias,
            carry_bias,
            cross_bias,
            self_finish_bias,
        )
    } else {
        GameStatePressure::new(
            score_deficit,
            total_remaining_seconds,
            urgency_index,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
        )
    }
}

pub fn analyze_match_state(state: &MatchState) -> GameStatePressure {
    let is_home_offense = state.possession().role().is_offense(state.home_team_id());
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

    let period = state.clock().period();
    let reg_periods = state.format_rules().regulation_periods();
    let seconds_in_period = state.clock().seconds_in_period();
    let period_duration_seconds = state.clock().period_duration_seconds();

    analyze_game_state(
        score_offense,
        score_defense,
        period,
        reg_periods,
        seconds_in_period,
        period_duration_seconds,
    )
}