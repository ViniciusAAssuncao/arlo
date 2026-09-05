use crate::possession::ball_state::BallState;
use crate::possession::clock_state::{ClockState, ClockStopReason};
use crate::possession::role::{opening_possession, PossessionRole};
use crate::possession::series_state::SeriesState;
use arlo_math::units::Position;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PossessionSnapshot {
    pub ball_state: BallState,
    pub clock_state: ClockState,
    pub role: PossessionRole,
    pub series_state: SeriesState,
}

impl PossessionSnapshot {
    pub fn new(
        ball_state: BallState,
        clock_state: ClockState,
        role: PossessionRole,
        series_state: SeriesState,
    ) -> Self {
        Self {
            ball_state,
            clock_state,
            role,
            series_state,
        }
    }

    pub fn opening(home_team: Uuid, away_team: Uuid, initial_scrimmage: Position) -> Self {
        Self {
            ball_state: BallState::Dead,
            clock_state: ClockState::Stopped(ClockStopReason::PeriodEnd),
            role: opening_possession(home_team, away_team),
            series_state: SeriesState::initial(initial_scrimmage),
        }
    }

    pub fn ball_state(&self) -> BallState {
        self.ball_state
    }

    pub fn clock_state(&self) -> ClockState {
        self.clock_state
    }

    pub fn role(&self) -> &PossessionRole {
        &self.role
    }

    pub fn series_state(&self) -> &SeriesState {
        &self.series_state
    }

    pub fn series_state_mut(&mut self) -> &mut SeriesState {
        &mut self.series_state
    }

    pub fn offense(&self) -> Uuid {
        self.role.offense()
    }

    pub fn defense(&self) -> Uuid {
        self.role.defense()
    }

    pub fn is_in_play(&self) -> bool {
        self.ball_state.is_in_play()
    }

    pub fn is_clock_running(&self) -> bool {
        self.clock_state.is_running()
    }

    pub fn down(&self) -> u8 {
        self.series_state.down()
    }

    pub fn advanced_mirins(&self) -> f64 {
        self.series_state.advanced_mirins()
    }

    pub fn scrimmage_point(&self) -> Position {
        self.series_state.scrimmage_point()
    }
}
