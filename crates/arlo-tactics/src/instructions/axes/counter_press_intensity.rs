use arlo_math::stats::UnipolarScalar;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct CounterPressIntensity(UnipolarScalar);

impl CounterPressIntensity {
    pub fn new_clamped(value: f64) -> Self {
        Self(UnipolarScalar::new_clamped(value))
    }

    pub fn value(&self) -> f64 {
        self.0.value()
    }
}
