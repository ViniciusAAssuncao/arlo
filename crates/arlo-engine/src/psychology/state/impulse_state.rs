use arlo_domain::sport_constants::IMPULSE_SCALE_MAX;
use serde::{Deserialize, Serialize};

pub const IMPULSE_HISTORY_CAPACITY: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct ImpulseHistoryEntry {
    pub is_positive: bool,
    pub magnitude: f64,
    pub timestamp_seconds: f64,
}

impl ImpulseHistoryEntry {
    pub fn new(is_positive: bool, magnitude: f64, timestamp_seconds: f64) -> Self {
        Self {
            is_positive,
            magnitude: magnitude.max(0.0),
            timestamp_seconds: timestamp_seconds.max(0.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ImpulseState {
    value: u8,
    accumulator: f64,
    recent_successes: u32,
    recent_failures: u32,
    consecutive_successes: u32,
    consecutive_failures: u32,
    history: [ImpulseHistoryEntry; IMPULSE_HISTORY_CAPACITY],
    history_count: usize,
    history_head: usize,
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
            history: [ImpulseHistoryEntry::default(); IMPULSE_HISTORY_CAPACITY],
            history_count: 0,
            history_head: 0,
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
            history: [ImpulseHistoryEntry::default(); IMPULSE_HISTORY_CAPACITY],
            history_count: 0,
            history_head: 0,
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

    pub fn history_count(&self) -> usize {
        self.history_count
    }

    pub fn history(&self) -> &[ImpulseHistoryEntry; IMPULSE_HISTORY_CAPACITY] {
        &self.history
    }

    pub fn recent_history(&self) -> Vec<ImpulseHistoryEntry> {
        let mut entries = Vec::with_capacity(self.history_count);
        for i in 0..self.history_count {
            let idx =
                (self.history_head + IMPULSE_HISTORY_CAPACITY - 1 - i) % IMPULSE_HISTORY_CAPACITY;
            entries.push(self.history[idx]);
        }
        entries
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

    pub fn record_event(&mut self, is_positive: bool, magnitude: f64, timestamp_seconds: f64) {
        if is_positive {
            self.record_success();
        } else {
            self.record_failure();
        }
        self.history[self.history_head] =
            ImpulseHistoryEntry::new(is_positive, magnitude, timestamp_seconds);
        self.history_head = (self.history_head + 1) % IMPULSE_HISTORY_CAPACITY;
        if self.history_count < IMPULSE_HISTORY_CAPACITY {
            self.history_count += 1;
        }
    }

    pub fn reset_recent_counters(&mut self) {
        self.recent_successes = 0;
        self.recent_failures = 0;
        self.consecutive_successes = 0;
        self.consecutive_failures = 0;
        self.history = [ImpulseHistoryEntry::default(); IMPULSE_HISTORY_CAPACITY];
        self.history_count = 0;
        self.history_head = 0;
    }

    pub fn momentum_index(&self, current_time_seconds: f64) -> f64 {
        if self.history_count == 0 {
            return 0.0;
        }

        let mut weighted_sum = 0.0;
        let mut total_weight = 0.0;

        for i in 0..self.history_count {
            let idx =
                (self.history_head + IMPULSE_HISTORY_CAPACITY - 1 - i) % IMPULSE_HISTORY_CAPACITY;
            let entry = &self.history[idx];
            let dt = (current_time_seconds - entry.timestamp_seconds).max(0.0);
            let time_decay = (-dt / 120.0).exp();
            let rank_decay = 0.85_f64.powi(i as i32);
            let weight = time_decay * rank_decay;

            let sign = if entry.is_positive { 1.0 } else { -1.0 };
            weighted_sum += sign * entry.magnitude.clamp(0.1, 3.0) * weight;
            total_weight += weight;
        }

        let history_effect = if total_weight > 0.0 {
            weighted_sum / total_weight
        } else {
            0.0
        };

        let streak_effect = if self.consecutive_successes > 1 {
            0.15 * ((self.consecutive_successes - 1).min(5) as f64)
        } else if self.consecutive_failures > 1 {
            -0.20 * ((self.consecutive_failures - 1).min(5) as f64)
        } else {
            0.0
        };

        (history_effect * 0.70 + streak_effect * 0.30).clamp(-1.0, 1.0)
    }

    pub fn momentum_multiplier_for(&self, is_positive: bool, momentum: f64) -> f64 {
        if is_positive {
            if momentum >= 0.0 {
                1.0 + 0.50 * momentum
            } else {
                (1.0 - 0.35 * (-momentum)).clamp(0.40, 1.0)
            }
        } else if momentum <= 0.0 {
            1.0 + 0.60 * (-momentum)
        } else {
            (1.0 - 0.40 * momentum).clamp(0.40, 1.0)
        }
    }
}

impl Default for ImpulseState {
    fn default() -> Self {
        Self::initial()
    }
}

pub type MentalState = ImpulseState;
