use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MarginContext {
    pub advantage_gp_equivalents: f64,
    pub strength_z_gap: f64,
}

impl MarginContext {
    pub fn new(advantage_gp_equivalents: f64, strength_z_gap: f64) -> Self {
        Self {
            advantage_gp_equivalents,
            strength_z_gap,
        }
    }

    pub fn from_scores(
        offense_score: u32,
        defense_score: u32,
        gp_point_value: f64,
        strength_z_gap: f64,
    ) -> Self {
        let diff = (offense_score as f64) - (defense_score as f64);
        let gp_val = if gp_point_value > 0.0 {
            gp_point_value
        } else {
            5.0
        };
        Self {
            advantage_gp_equivalents: diff / gp_val,
            strength_z_gap,
        }
    }

    pub fn advantage_gp_equivalents(&self) -> f64 {
        self.advantage_gp_equivalents
    }

    pub fn lead_gp_equivalents(&self) -> f64 {
        self.advantage_gp_equivalents
    }

    pub fn strength_z_gap(&self) -> f64 {
        self.strength_z_gap
    }
}
