use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TeamStrengthProfile {
    pub z_gap_gain: f64,
    pub artrine_individual_weight: f64,
    pub passer_individual_weight: f64,
    pub goalguard_individual_weight: f64,
    pub default_individual_weight: f64,
}

impl Default for TeamStrengthProfile {
    fn default() -> Self {
        Self {
            z_gap_gain: 1.2,
            artrine_individual_weight: 0.60,
            passer_individual_weight: 0.60,
            goalguard_individual_weight: 0.70,
            default_individual_weight: 0.30,
        }
    }
}