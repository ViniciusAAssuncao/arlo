use crate::scoring_model::difficulty_curves::calculate_base_difficulty_logit;
use crate::scoring_model::scoring_kind::ScoringKind;
use crate::scoring_model::scoring_situation::ScoringSituation;
use crate::scoring_model::tuning::ScoringDifficultyProfile;
use arlo_math::stats::contrast::logistic;
use arlo_math::Probability;

pub fn calculate_scoring_probability(
    kind: ScoringKind,
    situation: &ScoringSituation,
    profile: &ScoringDifficultyProfile,
) -> Probability {
    let base_logit = calculate_base_difficulty_logit(kind, situation, profile);
    let rating_diff = situation.finisher_rating - situation.goalguard_rating;
    let rating_shift = rating_diff * profile.rating_shift_weight(kind);

    let total_logit = base_logit + rating_shift;
    Probability::new_clamped(logistic(total_logit).clamp(0.01, 0.95))
}