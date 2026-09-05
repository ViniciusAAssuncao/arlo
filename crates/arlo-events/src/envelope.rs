use crate::MatchEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct MatchClockInstant {
    period: u32,
    seconds_in_period: f64,
}

impl MatchClockInstant {
    pub fn new(period: u32, seconds_in_period: f64) -> Self {
        Self {
            period,
            seconds_in_period,
        }
    }

    pub fn from_minutes_seconds(period: u32, minutes: u32, seconds: f64) -> Self {
        Self {
            period,
            seconds_in_period: (minutes as f64) * 60.0 + seconds,
        }
    }

    pub fn zero() -> Self {
        Self {
            period: 1,
            seconds_in_period: 0.0,
        }
    }

    pub fn period(&self) -> u32 {
        self.period
    }

    pub fn seconds_in_period(&self) -> f64 {
        self.seconds_in_period
    }

    pub fn is_overtime(&self) -> bool {
        self.period > 4
    }

    pub fn total_elapsed_seconds(&self) -> f64 {
        if self.period <= 1 {
            self.seconds_in_period
        } else if self.period <= 4 {
            ((self.period - 1) as f64) * 30.0 * 60.0 + self.seconds_in_period
        } else {
            4.0 * 30.0 * 60.0
                + ((self.period - 5) as f64) * 15.0 * 60.0
                + self.seconds_in_period
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchEventEnvelope<T = MatchEvent> {
    sequence_number: u64,
    clock: MatchClockInstant,
    payload: T,
}

impl<T> MatchEventEnvelope<T> {
    pub fn new(sequence_number: u64, clock: MatchClockInstant, payload: T) -> Self {
        Self {
            sequence_number,
            clock,
            payload,
        }
    }

    pub fn sequence_number(&self) -> u64 {
        self.sequence_number
    }

    pub fn clock(&self) -> MatchClockInstant {
        self.clock
    }

    pub fn payload(&self) -> &T {
        &self.payload
    }

    pub fn payload_mut(&mut self) -> &mut T {
        &mut self.payload
    }

    pub fn event(&self) -> &T {
        &self.payload
    }

    pub fn into_payload(self) -> T {
        self.payload
    }

    pub fn into_parts(self) -> (u64, MatchClockInstant, T) {
        (self.sequence_number, self.clock, self.payload)
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> MatchEventEnvelope<U> {
        MatchEventEnvelope {
            sequence_number: self.sequence_number,
            clock: self.clock,
            payload: f(self.payload),
        }
    }
}