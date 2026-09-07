use arlo_domain::sport_constants::IMPULSE_SCALE_MAX;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ImpulseState {
    value: u8,
    accumulator: f64,
    recent_successes: u32,
    recent_failures: u32,
    consecutive_successes: u32,
    consecutive_failures: u32,
}

impl ImpulseState {
    pub fn new(
        accumulator: f64,
        recent_successes: u32,
        recent_failures: u32,
        consecutive_successes: u32,
        consecutive_failures: u32,
    ) -> Self {
        let clamped_acc = accumulator.clamp(0.0, IMPULSE_SCALE_MAX as f64);
        let value = clamped_acc.round() as u8;
        Self {
            value,
            accumulator: clamped_acc,
            recent_successes,
            recent_failures,
            consecutive_successes,
            consecutive_failures,
        }
    }

    pub fn from_baseline(baseline: f64) -> Self {
        let clamped = baseline.clamp(0.0, IMPULSE_SCALE_MAX as f64);
        let value = clamped.round() as u8;
        Self {
            value,
            accumulator: clamped,
            recent_successes: 0,
            recent_failures: 0,
            consecutive_successes: 0,
            consecutive_failures: 0,
        }
    }

    pub fn initial() -> Self {
        Self::from_baseline(50.0)
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn impulse(&self) -> u8 {
        self.value
    }

    pub fn accumulator(&self) -> f64 {
        self.accumulator
    }

    pub fn recent_successes(&self) -> u32 {
        self.recent_successes
    }

    pub fn recent_failures(&self) -> u32 {
        self.recent_failures
    }

    pub fn consecutive_successes(&self) -> u32 {
        self.consecutive_successes
    }

    pub fn consecutive_failures(&self) -> u32 {
        self.consecutive_failures
    }

    pub fn set_accumulator(&mut self, new_accumulator: f64) {
        self.accumulator = new_accumulator.clamp(0.0, IMPULSE_SCALE_MAX as f64);
        let rounded = self.accumulator.round() as u8;
        if (self.accumulator - (self.value as f64)).abs() >= 0.5 {
            self.value = rounded;
        }
    }

    pub fn record_success(&mut self) {
        self.recent_successes = self.recent_successes.saturating_add(1);
        self.consecutive_successes = self.consecutive_successes.saturating_add(1);
        self.consecutive_failures = 0;
    }

    pub fn record_failure(&mut self) {
        self.recent_failures = self.recent_failures.saturating_add(1);
        self.consecutive_failures = self.consecutive_failures.saturating_add(1);
        self.consecutive_successes = 0;
    }

    pub fn reset_recent_counters(&mut self) {
        self.recent_successes = 0;
        self.recent_failures = 0;
        self.consecutive_successes = 0;
        self.consecutive_failures = 0;
    }
}

impl Default for ImpulseState {
    fn default() -> Self {
        Self::initial()
    }
}

pub type MentalState = ImpulseState;