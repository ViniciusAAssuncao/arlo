use arlo_domain::sport_constants::{
    ATTRIBUTE_MAX, ATTRIBUTE_MIN, SIGNAL_DETECTION_BASE_SENSITIVITY,
    SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
};
use arlo_math::stats::Probability;
use serde::{Deserialize, Serialize};
use std::f64::consts::SQRT_2;

fn error_function(x: f64) -> f64 {
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let abs_x = x.abs();

    let p = 0.3275911;
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;

    let t = 1.0 / (1.0 + p * abs_x);
    let poly = ((((a5 * t + a4) * t + a3) * t + a2) * t + a1) * t;
    let y = 1.0 - poly * (-abs_x * abs_x).exp();

    sign * y
}

fn standard_normal_cdf(x: f64) -> f64 {
    0.5 * (1.0 + error_function(x / SQRT_2))
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SignalDetectionModel {
    sensitivity_d_prime: f64,
    criterion: f64,
}

impl SignalDetectionModel {
    pub fn new(sensitivity_d_prime: f64, criterion: f64) -> Self {
        Self {
            sensitivity_d_prime: sensitivity_d_prime.max(0.0),
            criterion,
        }
    }

    pub fn from_judgment_attribute(judgment_attribute: f64, criterion: f64) -> Self {
        let norm_attr = judgment_attribute.clamp(ATTRIBUTE_MIN, ATTRIBUTE_MAX);
        let sensitivity_d_prime = (SIGNAL_DETECTION_BASE_SENSITIVITY
            + norm_attr * SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE)
            .max(0.0);
        Self::new(sensitivity_d_prime, criterion)
    }

    pub fn sensitivity_d_prime(&self) -> f64 {
        self.sensitivity_d_prime
    }

    pub fn criterion(&self) -> f64 {
        self.criterion
    }

    pub fn hit_rate(&self) -> Probability {
        let z = self.sensitivity_d_prime * 0.5 - self.criterion;
        Probability::new_clamped(standard_normal_cdf(z))
    }

    pub fn false_alarm_rate(&self) -> Probability {
        let z = -self.sensitivity_d_prime * 0.5 - self.criterion;
        Probability::new_clamped(standard_normal_cdf(z))
    }

    pub fn miss_rate(&self) -> Probability {
        self.hit_rate().complement()
    }

    pub fn correct_rejection_rate(&self) -> Probability {
        self.false_alarm_rate().complement()
    }
}

impl Default for SignalDetectionModel {
    fn default() -> Self {
        Self::new(SIGNAL_DETECTION_BASE_SENSITIVITY, 0.0)
    }
}
