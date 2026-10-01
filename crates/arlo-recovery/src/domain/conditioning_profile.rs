use arlo_domain::domain::validation::validate_float_range;
use arlo_domain::error::DomainResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ConditioningProfile {
    readiness: f64,
}

impl ConditioningProfile {
    pub fn new(readiness: f64) -> DomainResult<Self> {
        validate_float_range(readiness, 0.0, 1.0, "readiness")?;

        Ok(Self { readiness })
    }

    pub fn readiness(&self) -> f64 {
        self.readiness
    }
}
