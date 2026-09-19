use crate::scoring_model::margin::MarginPenaltyProfile;
use crate::scoring_model::scoring_kind::ScoringKind;
use arlo_events::ScoringPost;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ScoringDifficultyProfile {
    pub goal_point_intercept: f64,
    pub goal_point_distance_weight: f64,
    pub defense_closed_penalty: f64,
    pub field_point_intercept: f64,
    pub field_point_distance_weight: f64,
    pub field_goal_goalpost_intercept: f64,
    pub field_goal_fieldpost_intercept: f64,
    pub field_goal_distance_weight: f64,
    pub kick_foul_origin_penalty: f64,
    pub goal_point_rating_shift_weight: f64,
    pub field_point_rating_shift_weight: f64,
    pub field_goal_rating_shift_weight: f64,
    pub rating_gap_saturation_point: f64,
    pub rating_gap_slope: f64,
    pub margin_penalty: MarginPenaltyProfile,
}

impl ScoringDifficultyProfile {
    pub fn new(
        goal_point_intercept: f64,
        goal_point_distance_weight: f64,
        defense_closed_penalty: f64,
        field_point_intercept: f64,
        field_point_distance_weight: f64,
        field_goal_goalpost_intercept: f64,
        field_goal_fieldpost_intercept: f64,
        field_goal_distance_weight: f64,
        kick_foul_origin_penalty: f64,
        goal_point_rating_shift_weight: f64,
        field_point_rating_shift_weight: f64,
        field_goal_rating_shift_weight: f64,
        rating_gap_saturation_point: f64,
        rating_gap_slope: f64,
    ) -> Self {
        Self {
            goal_point_intercept,
            goal_point_distance_weight,
            defense_closed_penalty,
            field_point_intercept,
            field_point_distance_weight,
            field_goal_goalpost_intercept,
            field_goal_fieldpost_intercept,
            field_goal_distance_weight,
            kick_foul_origin_penalty,
            goal_point_rating_shift_weight,
            field_point_rating_shift_weight,
            field_goal_rating_shift_weight,
            rating_gap_saturation_point,
            rating_gap_slope,
            margin_penalty: MarginPenaltyProfile::default(),
        }
    }

    pub fn with_margin_penalty(mut self, margin_penalty: MarginPenaltyProfile) -> Self {
        self.margin_penalty = margin_penalty;
        self
    }

    pub fn calibrate_field_goal_intercepts(
        &mut self,
        target_gp_prob: f64,
        target_fp_prob: f64,
        distance_mirim: f64,
    ) {
        let gp_logit = (target_gp_prob / (1.0 - target_gp_prob)).ln();
        self.field_goal_goalpost_intercept =
            gp_logit + distance_mirim * self.field_goal_distance_weight;
        let fp_logit = (target_fp_prob / (1.0 - target_fp_prob)).ln();
        self.field_goal_fieldpost_intercept =
            fp_logit + distance_mirim * self.field_goal_distance_weight;
    }

    pub fn with_rating_gap_scaling(
        mut self,
        rating_gap_saturation_point: f64,
        rating_gap_slope: f64,
    ) -> Self {
        self.rating_gap_saturation_point = rating_gap_saturation_point;
        self.rating_gap_slope = rating_gap_slope;
        self
    }

    pub fn goal_point_intercept(&self) -> f64 {
        self.goal_point_intercept
    }

    pub fn goal_point_distance_weight(&self) -> f64 {
        self.goal_point_distance_weight
    }

    pub fn defense_closed_penalty(&self) -> f64 {
        self.defense_closed_penalty
    }

    pub fn field_point_intercept(&self) -> f64 {
        self.field_point_intercept
    }

    pub fn field_point_distance_weight(&self) -> f64 {
        self.field_point_distance_weight
    }

    pub fn field_goal_distance_weight(&self) -> f64 {
        self.field_goal_distance_weight
    }

    pub fn rating_gap_saturation_point(&self) -> f64 {
        self.rating_gap_saturation_point
    }

    pub fn rating_gap_slope(&self) -> f64 {
        self.rating_gap_slope
    }

    pub fn rating_shift_weight(&self, kind: ScoringKind) -> f64 {
        match kind {
            ScoringKind::GoalPoint => self.goal_point_rating_shift_weight,
            ScoringKind::FieldPoint => self.field_point_rating_shift_weight,
            ScoringKind::FieldGoal(_) => self.field_goal_rating_shift_weight,
        }
    }

    pub fn field_goal_intercept(&self, post: ScoringPost) -> f64 {
        match post {
            ScoringPost::Goalpost => self.field_goal_goalpost_intercept,
            ScoringPost::Fieldpost => self.field_goal_fieldpost_intercept,
        }
    }

    pub fn margin_penalty(&self) -> &MarginPenaltyProfile {
        &self.margin_penalty
    }
}
