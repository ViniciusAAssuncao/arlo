use arlo_domain::{KickFoulScoringTier, PitchZone};

pub fn determine_kick_foul_scoring_tier(zone: PitchZone) -> KickFoulScoringTier {
    KickFoulScoringTier::from_zone(zone)
}
