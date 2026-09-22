use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DriveAwardProfile {
    base_logit: f64,
    advantage_scale: f64,
    skill_scale: f64,
    min_prob: f64,
    max_prob: f64,
}

impl DriveAwardProfile {
    pub fn new(
        base_logit: f64,
        advantage_scale: f64,
        skill_scale: f64,
        min_prob: f64,
        max_prob: f64,
    ) -> Self {
        Self {
            base_logit,
            advantage_scale,
            skill_scale,
            min_prob,
            max_prob,
        }
    }

    pub fn base_logit(&self) -> f64 {
        self.base_logit
    }

    pub fn advantage_scale(&self) -> f64 {
        self.advantage_scale
    }

    pub fn skill_scale(&self) -> f64 {
        self.skill_scale
    }

    pub fn min_prob(&self) -> f64 {
        self.min_prob
    }

    pub fn max_prob(&self) -> f64 {
        self.max_prob
    }
}

impl Default for DriveAwardProfile {
    fn default() -> Self {
        Self {
            base_logit: 1.20,
            advantage_scale: 0.25,
            skill_scale: 0.40,
            min_prob: 0.20,
            max_prob: 0.95,
        }
    }
}
