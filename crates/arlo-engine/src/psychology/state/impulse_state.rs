use arlo_domain::sport_constants::IMPULSE_SCALE_MAX;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ImpulseState {
    accumulator: f64,
    baseline: f64,
    momentum: f64,
}

impl ImpulseState {
    pub fn new(accumulator: f64, baseline: f64, momentum: f64) -> Self {
        Self {
            accumulator: accumulator.clamp(0.0, IMPULSE_SCALE_MAX as f64),
            baseline: baseline.clamp(0.0, IMPULSE_SCALE_MAX as f64),
            momentum: momentum.clamp(-1.0, 1.0),
        }
    }

    pub fn from_baseline(baseline: f64) -> Self {
        let clamped = baseline.clamp(0.0, IMPULSE_SCALE_MAX as f64);
        Self {
            accumulator: clamped,
            baseline: clamped,
            momentum: 0.0,
        }
    }

    pub fn initial() -> Self {
        Self::from_baseline(50.0)
    }

    pub fn value(&self) -> u8 {
        self.accumulator.clamp(0.0, IMPULSE_SCALE_MAX as f64).round() as u8
    }

    pub fn impulse(&self) -> u8 {
        self.value()
    }

    pub fn accumulator(&self) -> f64 {
        self.accumulator
    }

    pub fn baseline(&self) -> f64 {
        self.baseline
    }

    pub fn momentum(&self) -> f64 {
        self.momentum
    }

    pub fn momentum_index(&self) -> f64 {
        self.momentum
    }

    pub fn momentum_index_at(&self, _current_time: f64) -> f64 {
        self.momentum
    }

    pub fn set_accumulator(&mut self, accumulator: f64) {
        self.accumulator = accumulator.clamp(0.0, IMPULSE_SCALE_MAX as f64);
    }

    pub fn set_baseline(&mut self, baseline: f64) {
        self.baseline = baseline.clamp(0.0, IMPULSE_SCALE_MAX as f64);
    }

    pub fn set_momentum(&mut self, momentum: f64) {
        self.momentum = momentum.clamp(-1.0, 1.0);
    }

    pub fn update_momentum(&mut self, event_signal: f64, alpha: f64) {
        let a = alpha.clamp(0.0, 1.0);
        self.momentum = (self.momentum * (1.0 - a) + event_signal * a).clamp(-1.0, 1.0);
    }

    pub fn momentum_multiplier_for(&self, is_positive: bool) -> f64 {
        if is_positive {
            if self.momentum >= 0.0 {
                1.0 + 0.50 * self.momentum
            } else {
                (1.0 - 0.35 * (-self.momentum)).clamp(0.40, 1.0)
            }
        } else if self.momentum <= 0.0 {
            1.0 + 0.60 * (-self.momentum)
        } else {
            (1.0 - 0.40 * self.momentum).clamp(0.40, 1.0)
        }
    }
}

impl Default for ImpulseState {
    fn default() -> Self {
        Self::initial()
    }
}

pub type MentalState = ImpulseState;
