use crate::world_state::constants::*;
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
            ArtrineDecisionKind::SelfCarry => {
                if drives_in_series < DRIVES_THRESHOLD_FOR_SCORING_OPPORTUNITY {
                    self.carry_bias
                        * (1.0
                            + CARRY_EARLY_DRIVE_BONUS_MULTIPLIER
                                * ((DRIVES_THRESHOLD_FOR_SCORING_OPPORTUNITY - drives_in_series)
                                    as f64))
                } else {
                    self.carry_bias
                }
            }
            ArtrineDecisionKind::ShortPass => self.short_pass_bias,
            ArtrineDecisionKind::LongLaunch => self.long_launch_bias,
            ArtrineDecisionKind::Cross => {
                if drives_in_series >= DRIVES_THRESHOLD_FOR_SCORING_OPPORTUNITY {
                    self.cross_bias * self.goal_point_bias
                } else {
                    self.cross_bias * self.field_point_bias
                }
            }
            ArtrineDecisionKind::SelfFinish => {
                if drives_in_series >= DRIVES_THRESHOLD_FOR_SCORING_OPPORTUNITY {
                    self.self_finish_bias * self.goal_point_bias
                } else {
                    self.self_finish_bias * self.field_point_bias
                }
            }
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
            goal_point_bias: DEFAULT_PRESSURE_BIAS,
            field_point_bias: DEFAULT_PRESSURE_BIAS,
            turnover_aversion_scale: DEFAULT_PRESSURE_BIAS,
            long_launch_bias: DEFAULT_PRESSURE_BIAS,
            short_pass_bias: DEFAULT_PRESSURE_BIAS,
            carry_bias: DEFAULT_PRESSURE_BIAS,
            cross_bias: DEFAULT_PRESSURE_BIAS,
            self_finish_bias: DEFAULT_PRESSURE_BIAS,
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

    let time_urgency = 1.0
        / (1.0
            + (total_remaining_seconds / URGENCY_TIME_HALF_LIFE_SECONDS).powf(URGENCY_POWER_CURVE));

    let urgency_index = if score_deficit > 0 {
        ((score_deficit as f64)
            * TRAILING_URGENCY_DEFICIT_FACTOR
            * (TRAILING_URGENCY_BASE_WEIGHT + TRAILING_URGENCY_TIME_WEIGHT * time_urgency))
            .clamp(0.0, MAX_TRAILING_URGENCY_INDEX)
    } else if score_deficit < 0 {
        ((-score_deficit as f64) * LEADING_URGENCY_DEFICIT_FACTOR * time_urgency)
            .clamp(0.0, MAX_LEADING_URGENCY_INDEX)
    } else {
        (TIED_URGENCY_BASE_FACTOR * time_urgency).clamp(0.0, MAX_TIED_URGENCY_INDEX)
    };

    if score_deficit >= LARGE_DEFICIT_THRESHOLD && time_urgency > URGENCY_THRESHOLD {
        let intensity =
            ((time_urgency - URGENCY_THRESHOLD) / URGENCY_INTENSITY_DIVISOR).clamp(0.0, 1.0);
        let goal_point_bias = (1.0 + LARGE_DEFICIT_GOAL_POINT_SCALE * intensity)
            .clamp(1.0, LARGE_DEFICIT_GOAL_POINT_MAX);
        let field_point_bias = (1.0 - LARGE_DEFICIT_FIELD_POINT_SCALE * intensity)
            .clamp(LARGE_DEFICIT_FIELD_POINT_MIN, 1.00);
        let long_launch_bias = (1.0 + LARGE_DEFICIT_LONG_LAUNCH_SCALE * intensity)
            .clamp(1.0, LARGE_DEFICIT_LONG_LAUNCH_MAX);
        let cross_bias =
            (1.0 + LARGE_DEFICIT_CROSS_SCALE * intensity).clamp(1.0, LARGE_DEFICIT_CROSS_MAX);
        let self_finish_bias = (1.0 + LARGE_DEFICIT_SELF_FINISH_SCALE * intensity)
            .clamp(1.0, LARGE_DEFICIT_SELF_FINISH_MAX);
        let short_pass_bias = (1.0 - LARGE_DEFICIT_SHORT_PASS_SCALE * intensity)
            .clamp(LARGE_DEFICIT_SHORT_PASS_MIN, 1.00);
        let carry_bias =
            (1.0 - LARGE_DEFICIT_CARRY_SCALE * intensity).clamp(LARGE_DEFICIT_CARRY_MIN, 1.00);
        let turnover_aversion_scale = (1.0 - LARGE_DEFICIT_TURNOVER_AVERSION_SCALE * intensity)
            .clamp(LARGE_DEFICIT_TURNOVER_AVERSION_MIN, 1.00);

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
    } else if score_deficit > 0
        && score_deficit < LARGE_DEFICIT_THRESHOLD
        && time_urgency > URGENCY_THRESHOLD
    {
        let intensity =
            ((time_urgency - URGENCY_THRESHOLD) / URGENCY_INTENSITY_DIVISOR).clamp(0.0, 1.0);
        let goal_point_bias = (1.0 + SMALL_DEFICIT_GOAL_POINT_SCALE * intensity)
            .clamp(1.0, SMALL_DEFICIT_GOAL_POINT_MAX);
        let field_point_bias = (1.0 + SMALL_DEFICIT_FIELD_POINT_SCALE * intensity)
            .clamp(1.0, SMALL_DEFICIT_FIELD_POINT_MAX);
        let long_launch_bias = (1.0 + SMALL_DEFICIT_LONG_LAUNCH_SCALE * intensity)
            .clamp(1.0, SMALL_DEFICIT_LONG_LAUNCH_MAX);
        let cross_bias =
            (1.0 + SMALL_DEFICIT_CROSS_SCALE * intensity).clamp(1.0, SMALL_DEFICIT_CROSS_MAX);
        let self_finish_bias = (1.0 + SMALL_DEFICIT_SELF_FINISH_SCALE * intensity)
            .clamp(1.0, SMALL_DEFICIT_SELF_FINISH_MAX);
        let short_pass_bias = (1.0 + SMALL_DEFICIT_SHORT_PASS_SCALE * intensity)
            .clamp(1.0, SMALL_DEFICIT_SHORT_PASS_MAX);
        let carry_bias =
            (1.0 + SMALL_DEFICIT_CARRY_SCALE * intensity).clamp(1.0, SMALL_DEFICIT_CARRY_MAX);
        let turnover_aversion_scale = (1.0 + SMALL_DEFICIT_TURNOVER_AVERSION_SCALE * intensity)
            .clamp(1.0, SMALL_DEFICIT_TURNOVER_AVERSION_MAX);

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
    } else if score_deficit < 0 && time_urgency > URGENCY_THRESHOLD {
        let intensity =
            ((time_urgency - URGENCY_THRESHOLD) / URGENCY_INTENSITY_DIVISOR).clamp(0.0, 1.0);
        let goal_point_bias =
            (1.0 - LEADING_GOAL_POINT_SCALE * intensity).clamp(LEADING_GOAL_POINT_MIN, 1.00);
        let field_point_bias = 1.0;
        let long_launch_bias =
            (1.0 - LEADING_LONG_LAUNCH_SCALE * intensity).clamp(LEADING_LONG_LAUNCH_MIN, 1.00);
        let cross_bias = (1.0 - LEADING_CROSS_SCALE * intensity).clamp(LEADING_CROSS_MIN, 1.00);
        let self_finish_bias =
            (1.0 - LEADING_SELF_FINISH_SCALE * intensity).clamp(LEADING_SELF_FINISH_MIN, 1.00);
        let short_pass_bias =
            (1.0 + LEADING_SHORT_PASS_SCALE * intensity).clamp(1.0, LEADING_SHORT_PASS_MAX);
        let carry_bias = (1.0 + LEADING_CARRY_SCALE * intensity).clamp(1.0, LEADING_CARRY_MAX);
        let turnover_aversion_scale = (1.0 + LEADING_TURNOVER_AVERSION_SCALE * intensity)
            .clamp(1.0, LEADING_TURNOVER_AVERSION_MAX);

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
            DEFAULT_PRESSURE_BIAS,
            DEFAULT_PRESSURE_BIAS,
            DEFAULT_PRESSURE_BIAS,
            DEFAULT_PRESSURE_BIAS,
            DEFAULT_PRESSURE_BIAS,
            DEFAULT_PRESSURE_BIAS,
            DEFAULT_PRESSURE_BIAS,
            DEFAULT_PRESSURE_BIAS,
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
