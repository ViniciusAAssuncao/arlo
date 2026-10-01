use crate::error::{EngineError, EngineResult};
use arlo_domain::sport_constants::{
    MAX_ADDED_TIME_SECONDS, PERIOD_DURATION_MINUTES, REGULAR_PERIODS_COUNT,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClockState {
    period: u32,
    seconds_in_period: f64,
    total_elapsed_seconds: f64,
    added_seconds: f64,
    running: bool,
}

impl Default for ClockState {
    fn default() -> Self {
        Self {
            period: 1,
            seconds_in_period: 0.0,
            total_elapsed_seconds: 0.0,
            added_seconds: 0.0,
            running: false,
        }
    }
}

impl ClockState {
    pub fn period(&self) -> u32 {
        self.period
    }
    pub fn seconds_in_period(&self) -> f64 {
        self.seconds_in_period
    }
    pub fn total_elapsed_seconds(&self) -> f64 {
        self.total_elapsed_seconds
    }
    pub fn added_seconds(&self) -> f64 {
        self.added_seconds
    }
    pub fn is_running(&self) -> bool {
        self.running
    }
    pub fn period_limit_seconds(&self) -> f64 {
        f64::from(PERIOD_DURATION_MINUTES) * 60.0 + self.added_seconds
    }

    pub fn maximum_period_seconds(&self) -> f64 {
        f64::from(PERIOD_DURATION_MINUTES) * 60.0 + MAX_ADDED_TIME_SECONDS
    }

    pub fn start(self) -> EngineResult<Self> {
        if self.running || self.seconds_in_period >= self.period_limit_seconds() {
            return Err(EngineError::InvalidTransition(
                "clock cannot start at this boundary".into(),
            ));
        }
        Ok(Self {
            running: true,
            ..self
        })
    }

    pub fn stop(self) -> Self {
        Self {
            running: false,
            ..self
        }
    }

    pub fn advance(self, seconds: f64) -> EngineResult<Self> {
        if !self.running || !seconds.is_finite() || seconds < 0.0 {
            return Err(EngineError::InvalidTransition(
                "clock advance requires nonnegative finite playing time".into(),
            ));
        }
        let next = self.seconds_in_period + seconds;
        if next > self.period_limit_seconds() || !next.is_finite() {
            return Err(EngineError::InvalidTransition(
                "clock advance exceeds quarter limit".into(),
            ));
        }
        Ok(Self {
            seconds_in_period: next,
            total_elapsed_seconds: self.total_elapsed_seconds + seconds,
            ..self
        })
    }

    pub fn grant_added_time(self, seconds: f64) -> EngineResult<Self> {
        if !seconds.is_finite() || seconds <= self.added_seconds || seconds > MAX_ADDED_TIME_SECONDS
        {
            return Err(EngineError::InvalidTransition(
                "invalid added playing time".into(),
            ));
        }
        Ok(Self {
            added_seconds: seconds,
            ..self
        })
    }

    pub fn next_quarter(self) -> EngineResult<Self> {
        if self.running
            || self.period >= REGULAR_PERIODS_COUNT
            || self.seconds_in_period < self.period_limit_seconds()
        {
            return Err(EngineError::InvalidTransition(
                "quarter has not ended or match is complete".into(),
            ));
        }
        Ok(Self {
            period: self.period + 1,
            seconds_in_period: 0.0,
            added_seconds: 0.0,
            running: false,
            ..self
        })
    }
}
