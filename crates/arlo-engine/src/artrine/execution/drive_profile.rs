use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DriveAwardProfile {
    min_advance_p1: f64,
    p1_base_logit: f64,
    p1_advantage_scale: f64,
    p1_skill_scale: f64,
    p1_min_prob: f64,
    p1_max_prob: f64,
    min_advance_p2: f64,
    p2_base_logit: f64,
    p2_advantage_scale: f64,
    p2_skill_scale: f64,
    p2_min_prob: f64,
    p2_max_prob: f64,
    min_advance_p3: f64,
    p3_base_logit: f64,
    p3_advantage_scale: f64,
    p3_skill_scale: f64,
    p3_min_prob: f64,
    p3_max_prob: f64,
}

impl DriveAwardProfile {
    pub fn new(
        min_advance_p1: f64,
        p1_base_logit: f64,
        p1_advantage_scale: f64,
        p1_skill_scale: f64,
        p1_min_prob: f64,
        p1_max_prob: f64,
        min_advance_p2: f64,
        p2_base_logit: f64,
        p2_advantage_scale: f64,
        p2_skill_scale: f64,
        p2_min_prob: f64,
        p2_max_prob: f64,
        min_advance_p3: f64,
        p3_base_logit: f64,
        p3_advantage_scale: f64,
        p3_skill_scale: f64,
        p3_min_prob: f64,
        p3_max_prob: f64,
    ) -> Self {
        Self {
            min_advance_p1,
            p1_base_logit,
            p1_advantage_scale,
            p1_skill_scale,
            p1_min_prob,
            p1_max_prob,
            min_advance_p2,
            p2_base_logit,
            p2_advantage_scale,
            p2_skill_scale,
            p2_min_prob,
            p2_max_prob,
            min_advance_p3,
            p3_base_logit,
            p3_advantage_scale,
            p3_skill_scale,
            p3_min_prob,
            p3_max_prob,
        }
    }

    pub fn min_advance_p1(&self) -> f64 {
        self.min_advance_p1
    }

    pub fn p1_base_logit(&self) -> f64 {
        self.p1_base_logit
    }

    pub fn p1_advantage_scale(&self) -> f64 {
        self.p1_advantage_scale
    }

    pub fn p1_skill_scale(&self) -> f64 {
        self.p1_skill_scale
    }

    pub fn p1_min_prob(&self) -> f64 {
        self.p1_min_prob
    }

    pub fn p1_max_prob(&self) -> f64 {
        self.p1_max_prob
    }

    pub fn min_advance_p2(&self) -> f64 {
        self.min_advance_p2
    }

    pub fn p2_base_logit(&self) -> f64 {
        self.p2_base_logit
    }

    pub fn p2_advantage_scale(&self) -> f64 {
        self.p2_advantage_scale
    }

    pub fn p2_skill_scale(&self) -> f64 {
        self.p2_skill_scale
    }

    pub fn p2_min_prob(&self) -> f64 {
        self.p2_min_prob
    }

    pub fn p2_max_prob(&self) -> f64 {
        self.p2_max_prob
    }

    pub fn min_advance_p3(&self) -> f64 {
        self.min_advance_p3
    }

    pub fn p3_base_logit(&self) -> f64 {
        self.p3_base_logit
    }

    pub fn p3_advantage_scale(&self) -> f64 {
        self.p3_advantage_scale
    }

    pub fn p3_skill_scale(&self) -> f64 {
        self.p3_skill_scale
    }

    pub fn p3_min_prob(&self) -> f64 {
        self.p3_min_prob
    }

    pub fn p3_max_prob(&self) -> f64 {
        self.p3_max_prob
    }
}

impl Default for DriveAwardProfile {
    fn default() -> Self {
        Self {
            min_advance_p1: 1.0,
            p1_base_logit: -0.15,
            p1_advantage_scale: 0.32,
            p1_skill_scale: 0.45,
            p1_min_prob: 0.05,
            p1_max_prob: 0.95,
            min_advance_p2: 4.5,
            p2_base_logit: -1.75,
            p2_advantage_scale: 0.28,
            p2_skill_scale: 0.55,
            p2_min_prob: 0.01,
            p2_max_prob: 0.85,
            min_advance_p3: 8.5,
            p3_base_logit: -2.70,
            p3_advantage_scale: 0.25,
            p3_skill_scale: 0.65,
            p3_min_prob: 0.005,
            p3_max_prob: 0.70,
        }
    }
}
