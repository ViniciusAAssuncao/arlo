use arlo_domain::domain::validation::validate_float_range;
use arlo_domain::error::DomainResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FatigueCondition {
    energy: f64,
    w_prime: f64,
}

impl FatigueCondition {
    pub fn new(energy: f64, w_prime: f64) -> DomainResult<Self> {
        validate_float_range(energy, 0.0, 1.0, "energy")?;
        validate_float_range(w_prime, 0.0, 1.0, "w_prime")?;

        Ok(Self { energy, w_prime })
    }

    pub fn energy(&self) -> f64 {
        self.energy
    }

    pub fn w_prime(&self) -> f64 {
        self.w_prime
    }
}
