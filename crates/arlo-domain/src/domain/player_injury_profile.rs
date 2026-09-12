use crate::domain::sport_constants::injury::DEFAULT_INJURY_SUSCEPTIBILITY_MULTIPLIER;
use crate::domain::validation::validate_positive_finite;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlayerInjuryProfile {
    injury_susceptibility_multiplier: f64,
}

impl PlayerInjuryProfile {
    pub fn new(injury_susceptibility_multiplier: f64) -> DomainResult<Self> {
        validate_positive_finite(
            injury_susceptibility_multiplier,
            "injury_susceptibility_multiplier",
        )?;
        Ok(Self {
            injury_susceptibility_multiplier,
        })
    }

    pub fn injury_susceptibility_multiplier(&self) -> f64 {
        self.injury_susceptibility_multiplier
    }
}

impl Default for PlayerInjuryProfile {
    fn default() -> Self {
        Self {
            injury_susceptibility_multiplier: DEFAULT_INJURY_SUSCEPTIBILITY_MULTIPLIER,
        }
    }
}