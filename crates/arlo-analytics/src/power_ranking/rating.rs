use crate::error::{AnalyticsError, AnalyticsResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PowerRating(f64);

impl PowerRating {
    pub fn new(value: f64) -> AnalyticsResult<Self> {
        if !value.is_finite() {
            return Err(AnalyticsError::NonFinite {
                field: "power_rating".into(),
            });
        }
        Ok(Self(value))
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TeamPowerSeed {
    team_id: Uuid,
    initial_rating: PowerRating,
}

impl TeamPowerSeed {
    pub fn new(team_id: Uuid, initial_rating: PowerRating) -> Self {
        Self {
            team_id,
            initial_rating,
        }
    }

    pub fn team_id(self) -> Uuid {
        self.team_id
    }

    pub fn initial_rating(self) -> PowerRating {
        self.initial_rating
    }
}
