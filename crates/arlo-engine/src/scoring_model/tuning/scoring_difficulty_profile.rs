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
        }
    }

    pub fn new_unscaled(
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
    ) -> Self {
        Self::new(
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
            8.0,
            1.0,
        )
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

    pub fn goal_point_distance_penalty_weight(&self) -> f64 {
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

    pub fn field_point_distance_penalty_weight(&self) -> f64 {
        self.field_point_distance_weight
    }

    pub fn field_goal_goalpost_intercept(&self) -> f64 {
        self.field_goal_goalpost_intercept
    }

    pub fn field_goal_fieldpost_intercept(&self) -> f64 {
        self.field_goal_fieldpost_intercept
    }

    pub fn field_goal_distance_weight(&self) -> f64 {
        self.field_goal_distance_weight
    }

    pub fn field_goal_distance_penalty_weight(&self) -> f64 {
        self.field_goal_distance_weight
    }

    pub fn kick_foul_origin_penalty(&self) -> f64 {
        self.kick_foul_origin_penalty
    }

    pub fn goal_point_rating_shift_weight(&self) -> f64 {
        self.goal_point_rating_shift_weight
    }

    pub fn field_point_rating_shift_weight(&self) -> f64 {
        self.field_point_rating_shift_weight
    }

    pub fn field_goal_rating_shift_weight(&self) -> f64 {
        self.field_goal_rating_shift_weight
    }

    pub fn rating_gap_saturation_point(&self) -> f64 {
        self.rating_gap_saturation_point
    }

    pub fn rating_gap_saturation_threshold(&self) -> f64 {
        self.rating_gap_saturation_point
    }

    pub fn rating_gap_slope(&self) -> f64 {
        self.rating_gap_slope
    }

    pub fn rating_gap_steepness(&self) -> f64 {
        self.rating_gap_slope
    }

    pub fn scale_rating_gap(&self, rating_gap: f64) -> f64 {
        crate::scoring_model::rating_gap_scaling::scale_rating_gap(
            rating_gap,
            self.rating_gap_saturation_point,
            self.rating_gap_slope,
        )
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
}