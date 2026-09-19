use crate::scoring_model::scoring_kind::ScoringKind;
use crate::scoring_model::scoring_origin::ScoringOrigin;
use crate::scoring_model::scoring_situation::ScoringSituation;

pub fn calculate_base_difficulty_logit(kind: ScoringKind, situation: &ScoringSituation) -> f64 {
    let p = situation.normalized_proximity.clamp(0.0, 1.0);
    let distance_factor = 1.0 - p;

    let origin_penalty = match situation.origin {
        ScoringOrigin::OpenPlay => 0.0,
        ScoringOrigin::KickFoul => 1.8,
    };

    match kind {
        ScoringKind::GoalPoint => {
            let intercept = -1.0;
            let distance_penalty = distance_factor * 5.0;
            let defense_penalty = if situation.defense_closed { 1.5 } else { 0.0 };

            intercept - distance_penalty - defense_penalty - origin_penalty
        }
        ScoringKind::FieldPoint => {
            let intercept = 0.5;
            let distance_penalty = distance_factor * 3.0;

            intercept - distance_penalty - origin_penalty
        }
        ScoringKind::FieldGoal(post) => {
            let intercept = match post {
                arlo_events::ScoringPost::Goalpost => -0.5,
                arlo_events::ScoringPost::Fieldpost => 1.0,
            };
            let distance_penalty = distance_factor * 3.5;

            intercept - distance_penalty - origin_penalty
        }
    }
}