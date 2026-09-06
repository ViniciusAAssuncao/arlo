use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct OffensiveGravity {
    multiplier: f64,
    finishing_threat: f64,
    creation_threat: f64,
    positional_factor: f64,
}

impl OffensiveGravity {
    pub fn new(
        multiplier: f64,
        finishing_threat: f64,
        creation_threat: f64,
        positional_factor: f64,
    ) -> Self {
        Self {
            multiplier,
            finishing_threat,
            creation_threat,
            positional_factor,
        }
    }

    pub fn multiplier(&self) -> f64 {
        self.multiplier
    }

    pub fn finishing_threat(&self) -> f64 {
        self.finishing_threat
    }

    pub fn creation_threat(&self) -> f64 {
        self.creation_threat
    }

    pub fn positional_factor(&self) -> f64 {
        self.positional_factor
    }
}

impl Default for OffensiveGravity {
    fn default() -> Self {
        Self {
            multiplier: 1.0,
            finishing_threat: 1.0,
            creation_threat: 1.0,
            positional_factor: 1.0,
        }
    }
}