use crate::possession::ball_state::BallState;
use crate::possession::clock_state::{ClockState, ClockStopReason};
use crate::possession::immediate_loss::is_immediate_loss;
use crate::possession::role::PossessionRole;
use crate::possession::snapshot::PossessionSnapshot;
use arlo_math::units::Position;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct PlayOutcome {
    pub turnover: Option<Uuid>,
    pub out_of_bounds: bool,
    pub arbitral_stoppage: bool,
    pub mirins_advanced: f64,
    pub last_valid_possession_point: Position,
    pub possession_control_seconds: Option<f64>,
    pub score_occurred: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransitionResult {
    pub snapshot: PossessionSnapshot,
    pub countdown_to_size_triggered: bool,
    pub next_scrimmage_point: Option<Position>,
}

pub fn handle_turnover_without_out(
    current: &PossessionSnapshot,
    new_offense: Uuid,
) -> TransitionResult {
    let new_role = PossessionRole::new(new_offense, current.role().offense());
    let new_snapshot = PossessionSnapshot::new(
        current.ball_state,
        current.clock_state,
        new_role,
        current.series_state.clone(),
    );

    TransitionResult {
        snapshot: new_snapshot,
        countdown_to_size_triggered: false,
        next_scrimmage_point: None,
    }
}

pub fn transition(current: &PossessionSnapshot, outcome: &PlayOutcome) -> TransitionResult {
    if let Some(new_offense) = outcome.turnover {
        if !outcome.out_of_bounds && !outcome.arbitral_stoppage {
            return handle_turnover_without_out(current, new_offense);
        }
    }

    if outcome.out_of_bounds || outcome.arbitral_stoppage {
        let mut updated_series = current.series_state.clone();
        updated_series.record_advance(outcome.mirins_advanced);

        let ball_state = if outcome.out_of_bounds {
            BallState::OutOfBounds
        } else {
            BallState::Dead
        };

        let clock_stop_reason = if outcome.score_occurred {
            ClockStopReason::PointScored
        } else if outcome.out_of_bounds {
            ClockStopReason::OutOfBounds
        } else {
            ClockStopReason::ArbitralStoppage
        };

        let next_scrimmage = outcome.last_valid_possession_point;

        let next_role = if let Some(turnover_team) = outcome.turnover {
            updated_series.reset(next_scrimmage);
            PossessionRole::new(turnover_team, current.role().offense())
        } else if updated_series.should_turnover_on_downs() {
            updated_series.reset(next_scrimmage);
            current.role().swap()
        } else if updated_series.has_achieved_target() {
            updated_series.reset(next_scrimmage);
            *current.role()
        } else {
            let is_immediate = outcome
                .possession_control_seconds
                .map(is_immediate_loss)
                .unwrap_or(false);

            if !is_immediate {
                updated_series.advance_down();
            }
            *current.role()
        };

        let new_snapshot = PossessionSnapshot::new(
            ball_state,
            ClockState::Stopped(clock_stop_reason),
            next_role,
            updated_series,
        );

        TransitionResult {
            snapshot: new_snapshot,
            countdown_to_size_triggered: true,
            next_scrimmage_point: Some(next_scrimmage),
        }
    } else {
        let mut updated_series = current.series_state.clone();
        updated_series.record_advance(outcome.mirins_advanced);

        let new_snapshot = PossessionSnapshot::new(
            BallState::InPlay,
            ClockState::Running,
            *current.role(),
            updated_series,
        );

        TransitionResult {
            snapshot: new_snapshot,
            countdown_to_size_triggered: false,
            next_scrimmage_point: None,
        }
    }
}