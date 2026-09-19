use crate::scoring_model::scoring_kind::ScoringKind;
use crate::scoring_model::scoring_origin::ScoringOrigin;
use crate::scoring_model::scoring_situation::ScoringSituation;
use crate::scoring_model::tuning::ScoringDifficultyProfile;

pub fn calculate_base_difficulty_logit(
    kind: ScoringKind,
    situation: &ScoringSituation,
    profile: &ScoringDifficultyProfile,
) -> f64 {
    let p = situation.normalized_proximity.clamp(0.0, 1.0);
    let distance_factor = 1.0 - p;

    let origin_penalty = match situation.origin {
        ScoringOrigin::OpenPlay => 0.0,
        ScoringOrigin::KickFoul => profile.kick_foul_origin_penalty,
    };

    match kind {
        ScoringKind::GoalPoint => {
            let intercept = profile.goal_point_intercept;
            let distance_penalty = distance_factor * profile.goal_point_distance_weight;
            let defense_penalty = if situation.defense_closed {
                profile.defense_closed_penalty
            } else {
                0.0
            };

            intercept - distance_penalty - defense_penalty - origin_penalty
        }
        ScoringKind::FieldPoint => {
            let intercept = profile.field_point_intercept;
            let distance_penalty = distance_factor * profile.field_point_distance_weight;

            intercept - distance_penalty - origin_penalty
        }
        ScoringKind::FieldGoal(post) => {
            let intercept = profile.field_goal_intercept(post);
            let distance_penalty = distance_factor * profile.field_goal_distance_weight;

            intercept - distance_penalty - origin_penalty
        }
    }
}