use crate::scoring_model::margin::MarginPenaltyProfile;
use crate::scoring_model::tuning::scoring_difficulty_profile::ScoringDifficultyProfile;

impl Default for ScoringDifficultyProfile {
    fn default() -> Self {
        Self {
            goal_point_intercept: -0.35,
            goal_point_distance_weight: 3.8,
            defense_closed_penalty: 1.1,
            field_point_intercept: -0.04,
            field_point_distance_weight: 3.8,
            field_goal_goalpost_intercept: -0.50,
            field_goal_fieldpost_intercept: -0.15,
            field_goal_distance_weight: 3.5,
            kick_foul_origin_penalty: 1.8,
            goal_point_rating_shift_weight: 0.25,
            field_point_rating_shift_weight: 0.25,
            field_goal_rating_shift_weight: 0.25,
            rating_gap_saturation_point: 8.0,
            rating_gap_slope: 1.0,
            margin_penalty: MarginPenaltyProfile::default(),
        }
    }
}
