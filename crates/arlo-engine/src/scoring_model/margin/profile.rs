use crate::scoring_model::scoring_kind::ScoringKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MarginPenaltyProfile {
    pub goal_point_weight: f64,
    pub field_point_weight: f64,
    pub field_goal_weight: f64,
    pub max_penalty_logit: f64,
    pub threshold_gp: f64,
    pub softplus_beta: f64,
    pub saturation_scale: f64,
    pub equilibrium_z_width: f64,
}

impl MarginPenaltyProfile {
    pub fn new(
        goal_point_weight: f64,
        field_point_weight: f64,
        field_goal_weight: f64,
        max_penalty_logit: f64,
        threshold_gp: f64,
        softplus_beta: f64,
        saturation_scale: f64,
        equilibrium_z_width: f64,
    ) -> Self {
        Self {
            goal_point_weight,
            field_point_weight,
            field_goal_weight,
            max_penalty_logit,
            threshold_gp,
            softplus_beta,
            saturation_scale,
            equilibrium_z_width,
        }
    }

    pub fn goal_point_weight(&self) -> f64 {
        self.goal_point_weight
    }

    pub fn field_point_weight(&self) -> f64 {
        self.field_point_weight
    }

    pub fn field_goal_weight(&self) -> f64 {
        self.field_goal_weight
    }

    pub fn max_penalty_logit(&self) -> f64 {
        self.max_penalty_logit
    }

    pub fn threshold_gp(&self) -> f64 {
        self.threshold_gp
    }

    pub fn softplus_beta(&self) -> f64 {
        self.softplus_beta
    }

    pub fn saturation_scale(&self) -> f64 {
        self.saturation_scale
    }

    pub fn equilibrium_z_width(&self) -> f64 {
        self.equilibrium_z_width
    }

    pub fn weight_for_kind(&self, kind: ScoringKind) -> f64 {
        match kind {
            ScoringKind::GoalPoint => self.goal_point_weight,
            ScoringKind::FieldPoint => self.field_point_weight,
            ScoringKind::FieldGoal(_) => self.field_goal_weight,
        }
    }
}

impl Default for MarginPenaltyProfile {
    fn default() -> Self {
        Self {
            goal_point_weight: 1.0,
            field_point_weight: 0.0,
            field_goal_weight: 0.45,
            max_penalty_logit: 2.5,
            threshold_gp: 2.0,
            softplus_beta: 2.0,
            saturation_scale: 2.0,
            equilibrium_z_width: 1.2,
        }
    }
}
