use crate::match_decision::scoring::ScoringOpportunity;
use crate::set_piece::select_kick_post;
use arlo_domain::KickFoulScoringTier;

pub fn evaluate_kick_foul_scoring_opportunity(
    tier: KickFoulScoringTier,
    finisher_rating: f64,
) -> ScoringOpportunity {
    match tier {
        KickFoulScoringTier::FirstZone => ScoringOpportunity::GoalPoint,
        KickFoulScoringTier::Standard => {
            let post = select_kick_post(finisher_rating, 0.0);
            ScoringOpportunity::FieldGoal(post)
        }
    }
}
