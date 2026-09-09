use crate::manager_ai::cognition::leverage::compute_leverage;
use crate::manager_ai::cognition::team_momentum::aggregate_team_momentum;
use crate::world_state::context_analyzer::analyze_game_state;
use crate::world_state::MatchState;
use arlo_domain::sport_constants::PROJECTION_MOMENTUM_TO_SCORE_SCALE;
use arlo_tactics::derive_drive_scarcity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct SituationalAwareness {
    leverage: f64,
    momentum: f64,
    projected_deficit_delta: f64,
}

impl SituationalAwareness {
    pub fn new(leverage: f64, momentum: f64, projected_deficit_delta: f64) -> Self {
        Self {
            leverage,
            momentum,
            projected_deficit_delta,
        }
    }

    pub fn leverage(&self) -> f64 {
        self.leverage
    }

    pub fn momentum(&self) -> f64 {
        self.momentum
    }

    pub fn projected_deficit_delta(&self) -> f64 {
        self.projected_deficit_delta
    }

    pub fn build(state: &MatchState, team_id: Uuid) -> Self {
        let is_home = team_id == state.home_team_id();
        let (my_score, opponent_score) = if is_home {
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
        let regulation_periods = state.format_rules().regulation_periods();
        let seconds_in_period = state.clock().seconds_in_period();
        let period_duration_seconds = state.clock().period_duration_seconds();

        let game_state_pressure = analyze_game_state(
            my_score,
            opponent_score,
            period,
            regulation_periods,
            seconds_in_period,
            period_duration_seconds,
        );

        let urgency_index = game_state_pressure.urgency_index();
        let total_remaining_seconds = game_state_pressure.total_remaining_seconds();

        let drive_scarcity_component = if state.possession().offense() == team_id {
            derive_drive_scarcity(state.drives_in_current_series())
        } else {
            0.0
        };

        let leverage = compute_leverage(
            my_score,
            opponent_score,
            urgency_index,
            drive_scarcity_component,
        );
        let momentum = aggregate_team_momentum(state, team_id);

        let period_duration = if period_duration_seconds > 0.0 {
            period_duration_seconds
        } else {
            1.0
        };

        let projected_deficit_delta = -momentum
            * (total_remaining_seconds / period_duration)
            * PROJECTION_MOMENTUM_TO_SCORE_SCALE;

        Self {
            leverage,
            momentum,
            projected_deficit_delta,
        }
    }
}
