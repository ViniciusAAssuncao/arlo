use arlo_domain::domain::validation::validate_float_range;
use arlo_domain::error::DomainResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ImpulseCondition {
    current: f64,
    baseline: f64,
}

impl ImpulseCondition {
    pub fn new(current: f64, baseline: f64) -> DomainResult<Self> {
        validate_float_range(current, 0.0, 100.0, "current")?;
        validate_float_range(baseline, 0.0, 100.0, "baseline")?;

        Ok(Self { current, baseline })
    }

    pub fn current(&self) -> f64 {
        self.current
    }

    pub fn baseline(&self) -> f64 {
        self.baseline
    }
}
