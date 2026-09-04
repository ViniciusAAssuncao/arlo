use crate::error::MathError;
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

    pub fn value(self) -> f64 {
        self.0
    }
}