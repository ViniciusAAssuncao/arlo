use crate::error::MathError;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Probability(f64);

impl Probability {
    pub fn new(value: f64) -> Result<Self, MathError> {
        if (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(MathError::InvalidProbability(value))
        }
    }

    pub fn new_clamped(value: f64) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    pub fn value(self) -> f64 {
        self.0
    }

    pub fn complement(self) -> Self {
        Self(1.0 - self.0)
    }

    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> bool {
        rng.gen_bool(self.0)
    }

    pub fn sample_with<R: Rng + ?Sized>(&self, rng: &mut R) -> bool {
        rng.gen_bool(self.0)
    }
}
