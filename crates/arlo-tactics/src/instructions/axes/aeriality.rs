use arlo_math::stats::BipolarScalar;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Aeriality(BipolarScalar);

impl Aeriality {
    pub fn new_clamped(value: f64) -> Self {
        Self(BipolarScalar::new_clamped(value))
    }

    pub fn value(&self) -> f64 {
        self.0.value()
    }
}