use crate::scoring_model::margin::MarginPenaltyProfile;
use crate::scoring_model::tuning::scoring_difficulty_profile::ScoringDifficultyProfile;

impl Default for ScoringDifficultyProfile {
    fn default() -> Self {
        Self {
            goal_point_intercept: 1.85,
            goal_point_distance_weight: 1.40,
            defense_closed_penalty: 0.35,
            field_point_intercept: 2.60,
            field_point_distance_weight: 0.85,
            field_goal_goalpost_intercept: 2.10,
            field_goal_fieldpost_intercept: 1.70,
            field_goal_distance_weight: 0.75,
            kick_foul_origin_penalty: 0.40,
            goal_point_rating_shift_weight: 0.20,
            field_point_rating_shift_weight: 0.15,
            field_goal_rating_shift_weight: 0.15,
            rating_gap_saturation_point: 10.0,
            rating_gap_slope: 1.2,
            margin_penalty: MarginPenaltyProfile::default(),
        }
    }
}