use crate::scoring_model::{select_post_for_field_goal, ScoringDifficultyProfile, ScoringSituation};
use arlo_events::ScoringPost;
use rand::Rng;

pub fn select_kick_post<R: Rng + ?Sized>(
    situation: &ScoringSituation,
    difficulty_profile: &ScoringDifficultyProfile,
    kicker_decisions: f64,
    rng: &mut R,
) -> ScoringPost {
    select_post_for_field_goal(situation, difficulty_profile, kicker_decisions, rng)
}