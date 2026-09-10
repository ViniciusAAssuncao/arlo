use crate::possession::ball_state::BallState;
use crate::possession::clock_state::{ClockState, ClockStopReason};
use crate::possession::immediate_loss::is_immediate_loss;
use crate::possession::role::PossessionRole;
use crate::possession::snapshot::PossessionSnapshot;
use crate::psychology::systems::event_bus::DispatchedImpulseEvent;
use crate::psychology::systems::instrumentation::instrument_transition;
use arlo_math::units::Position;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayOutcome {
    pub turnover: Option<Uuid>,
    pub out_of_bounds: bool,
    pub arbitral_stoppage: bool,
    pub mirins_advanced: f64,
    pub last_valid_possession_point: Position,
    pub possession_control_seconds: Option<f64>,
    pub score_occurred: bool,
    pub is_goal_point: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransitionResult {
    pub snapshot: PossessionSnapshot,
    pub countdown_to_size_triggered: bool,
    pub next_scrimmage_point: Option<Position>,
    pub impulse_events: Vec<DispatchedImpulseEvent>,
}

pub fn handle_turnover_without_out(
    current: &PossessionSnapshot,
    new_offense: Uuid,
) -> TransitionResult {
    let new_role = PossessionRole::new(new_offense, current.role().offense());
    let mut new_series = current.series_state.clone();
    new_series.is_bonus_phase = false;
    let new_snapshot = PossessionSnapshot::with_live_sequence(
        current.ball_state,
        current.clock_state,
        new_role,
        new_series,
        current.live_sequence.clone(),
    );

    let dummy_outcome = PlayOutcome {
        turnover: Some(new_offense),
        out_of_bounds: false,
        arbitral_stoppage: false,
        mirins_advanced: 0.0,
        last_valid_possession_point: current.scrimmage_point(),
        possession_control_seconds: None,
        score_occurred: false,
        is_goal_point: false,
    };

    let impulse_events = instrument_transition(&dummy_outcome, current, &new_snapshot);

    TransitionResult {
        snapshot: new_snapshot,
        countdown_to_size_triggered: false,
        next_scrimmage_point: None,
        impulse_events,
    }
}

pub fn transition(current: &PossessionSnapshot, outcome: &PlayOutcome) -> TransitionResult {
    let next_scrimmage = outcome.last_valid_possession_point;
    let was_bonus_phase = current.series_state.is_bonus_phase;

    if let Some(new_offense) = outcome.turnover {
        if !outcome.out_of_bounds && !outcome.arbitral_stoppage {
            return handle_turnover_without_out(current, new_offense);
        }
    }

    let mut updated_series = current.series_state.clone();
    updated_series.record_advance(outcome.mirins_advanced);

    if outcome.out_of_bounds || outcome.arbitral_stoppage {
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

        let next_role = if was_bonus_phase {
            updated_series.reset(next_scrimmage);
            current.role().swap()
        } else if outcome.is_goal_point {
            updated_series.reset(next_scrimmage);
            updated_series.is_bonus_phase = true;
            *current.role()
        } else if outcome.score_occurred {
            updated_series.reset(next_scrimmage);
            current.role().swap()
        } else if let Some(turnover_team) = outcome.turnover {
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
                updated_series.set_scrimmage_point(next_scrimmage);
            }
            *current.role()
        };

        let new_snapshot = PossessionSnapshot::with_live_sequence(
            ball_state,
            ClockState::Stopped(clock_stop_reason),
            next_role,
            updated_series,
            current.live_sequence.clone(),
        );

        let impulse_events = instrument_transition(outcome, current, &new_snapshot);

        TransitionResult {
            snapshot: new_snapshot,
            countdown_to_size_triggered: true,
            next_scrimmage_point: Some(next_scrimmage),
            impulse_events,
        }
    } else {
        let (next_role, countdown) = if was_bonus_phase {
            updated_series.reset(next_scrimmage);
            (current.role().swap(), true)
        } else if outcome.is_goal_point {
            updated_series.reset(next_scrimmage);
            updated_series.is_bonus_phase = true;
            (*current.role(), true)
        } else if outcome.score_occurred {
            updated_series.reset(next_scrimmage);
            (current.role().swap(), true)
        } else if updated_series.has_achieved_target() {
            updated_series.reset(next_scrimmage);
            (*current.role(), false)
        } else if updated_series.should_turnover_on_downs() {
            updated_series.reset(next_scrimmage);
            (current.role().swap(), true)
        } else {
            updated_series.advance_down();
            updated_series.set_scrimmage_point(next_scrimmage);
            (*current.role(), false)
        };

        let new_snapshot = PossessionSnapshot::with_live_sequence(
            BallState::InPlay,
            ClockState::Running,
            next_role,
            updated_series,
            current.live_sequence.clone(),
        );

        let impulse_events = instrument_transition(outcome, current, &new_snapshot);

        TransitionResult {
            snapshot: new_snapshot,
            countdown_to_size_triggered: countdown,
            next_scrimmage_point: Some(next_scrimmage),
            impulse_events,
        }
    }
}
