use crate::match_decision::scoring::ScoringOpportunity;
use arlo_domain::KickFoulScoringTier;

pub fn evaluate_kick_foul_scoring_opportunity(
    tier: KickFoulScoringTier,
) -> ScoringOpportunity {
    match tier {
        KickFoulScoringTier::FirstZone => ScoringOpportunity::GoalPoint,
        KickFoulScoringTier::Standard => ScoringOpportunity::FieldPoint,
    }
}
