use crate::scoring_model::tuning::ScoringDifficultyProfile;
use crate::scoring_model::{calculate_scoring_probability, ScoringKind, ScoringSituation};
use arlo_events::ScoringPost;
use rand::Rng;

pub fn select_post_for_field_goal<R: Rng + ?Sized>(
    situation: &ScoringSituation,
    difficulty_profile: &ScoringDifficultyProfile,
    kicker_decisions: f64,
    rng: &mut R,
) -> ScoringPost {
    let gp_prob = calculate_scoring_probability(
        ScoringKind::FieldGoal(ScoringPost::Goalpost),
        situation,
        difficulty_profile,
    )
    .value();
    let fp_prob = calculate_scoring_probability(
        ScoringKind::FieldGoal(ScoringPost::Fieldpost),
        situation,
        difficulty_profile,
    )
    .value();

    let gp_ev = gp_prob * (arlo_domain::sport_constants::FIELD_GOAL_GOALPOST_VALUE as f64);
    let fp_ev = fp_prob * (arlo_domain::sport_constants::FIELD_GOAL_FIELDPOST_VALUE as f64);

    let steepness = arlo_domain::sport_constants::decision_steepness_for(kicker_decisions);
    let weights = arlo_math::stats::softmax_weights(&[gp_ev, fp_ev], steepness);

    if arlo_math::stats::sample_categorical(&weights, rng).unwrap_or(0) == 0 {
        ScoringPost::Goalpost
    } else {
        ScoringPost::Fieldpost
    }
}
