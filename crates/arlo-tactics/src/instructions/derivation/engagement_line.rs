use crate::instructions::axes::{Compactness, DefensiveLineHeight};
use arlo_math::stats::BipolarScalar;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct EngagementLine(BipolarScalar);

impl EngagementLine {
    pub fn new_clamped(value: f64) -> Self {
        Self(BipolarScalar::new_clamped(value))
    }

    pub fn from_defensive_line_and_compactness(
        defensive_line_height: DefensiveLineHeight,
        compactness: Compactness,
    ) -> Self {
        let raw = defensive_line_height.value()
            + (1.0 - defensive_line_height.value()) * compactness.value();
        Self::new_clamped(raw)
    }

    pub fn value(&self) -> f64 {
        self.0.value()
    }
}
