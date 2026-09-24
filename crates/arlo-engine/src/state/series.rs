use crate::error::{EngineError, EngineResult};
use arlo_domain::sport_constants::{
    MAX_CALL_TO_ACTIONS_PER_SERIES, MINIMUM_ADVANCE_MIRINS_PER_SERIES,
};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SeriesAdvance {
    Continuing { distance_to_gain_mirim: f64 },
    FirstDown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeriesOut {
    Retained,
    NewSeries,
    TurnoverOnDowns,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SeriesState {
    team_id: Uuid,
    down: u8,
    distance_to_gain_mirim: f64,
    origin_mirim: f64,
    valid_advance_mirim: f64,
    suspended: bool,
    turnover_lock: bool,
    renewed_this_call: bool,
}

impl SeriesState {
    pub fn new(team_id: Uuid, origin_mirim: f64, pitch_length_mirim: f64) -> EngineResult<Self> {
        if !origin_mirim.is_finite()
            || !pitch_length_mirim.is_finite()
            || origin_mirim < 0.0
            || origin_mirim > pitch_length_mirim
        {
            return Err(EngineError::InvalidTransition(
                "invalid series origin".into(),
            ));
        }
        Ok(Self {
            team_id,
            down: 1,
            distance_to_gain_mirim: MINIMUM_ADVANCE_MIRINS_PER_SERIES,
            origin_mirim,
            valid_advance_mirim: 0.0,
            suspended: false,
            turnover_lock: false,
            renewed_this_call: false,
        })
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }
    pub fn down(&self) -> u8 {
        self.down
    }
    pub fn distance_to_gain_mirim(&self) -> f64 {
        self.distance_to_gain_mirim
    }
    pub fn origin_mirim(&self) -> f64 {
        self.origin_mirim
    }
    pub fn valid_advance_mirim(&self) -> f64 {
        self.valid_advance_mirim
    }
    pub fn is_suspended(&self) -> bool {
        self.suspended
    }
    pub fn is_advance_locked(&self) -> bool {
        self.turnover_lock
    }

    pub fn record_valid_advance(
        self,
        gain_mirim: f64,
        end_mirim: f64,
        pitch_length_mirim: f64,
    ) -> EngineResult<(Self, SeriesAdvance)> {
        if self.suspended
            || self.turnover_lock
            || !gain_mirim.is_finite()
            || !end_mirim.is_finite()
            || end_mirim < 0.0
            || end_mirim > pitch_length_mirim
        {
            return Err(EngineError::InvalidTransition(
                "invalid advance in current series".into(),
            ));
        }
        let total = self.valid_advance_mirim + gain_mirim;
        if !total.is_finite() {
            return Err(EngineError::InvalidTransition(
                "series advance overflow".into(),
            ));
        }
        if total >= MINIMUM_ADVANCE_MIRINS_PER_SERIES {
            return Ok((
                Self {
                    down: 1,
                    distance_to_gain_mirim: MINIMUM_ADVANCE_MIRINS_PER_SERIES,
                    origin_mirim: end_mirim,
                    valid_advance_mirim: 0.0,
                    renewed_this_call: true,
                    ..self
                },
                SeriesAdvance::FirstDown,
            ));
        }
        let remaining = MINIMUM_ADVANCE_MIRINS_PER_SERIES - total;
        Ok((
            Self {
                valid_advance_mirim: total,
                distance_to_gain_mirim: remaining,
                ..self
            },
            SeriesAdvance::Continuing {
                distance_to_gain_mirim: remaining,
            },
        ))
    }

    pub fn suspend_after_turnover(self) -> Self {
        Self {
            suspended: true,
            turnover_lock: true,
            ..self
        }
    }

    pub fn resume_after_recovery(self) -> Self {
        Self {
            suspended: false,
            ..self
        }
    }

    pub fn resume_for_new_call(self) -> Self {
        Self {
            suspended: false,
            turnover_lock: false,
            renewed_this_call: false,
            ..self
        }
    }

    pub fn advance_after_out(self) -> (Self, SeriesOut) {
        let next = Self {
            suspended: false,
            turnover_lock: false,
            renewed_this_call: false,
            ..self
        };
        if self.renewed_this_call {
            return (next, SeriesOut::Retained);
        }
        if u32::from(self.down) >= MAX_CALL_TO_ACTIONS_PER_SERIES {
            return (next, SeriesOut::TurnoverOnDowns);
        }
        (
            Self {
                down: self.down + 1,
                ..next
            },
            SeriesOut::Retained,
        )
    }
}
