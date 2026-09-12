use arlo_domain::pitch::Pitch;
use arlo_domain::KickFoulScoringTier;
use arlo_math::units::Position as VectorPosition;

pub fn determine_kick_foul_scoring_tier(
    pitch: &Pitch,
    spot: VectorPosition,
) -> KickFoulScoringTier {
    let zone = pitch.zone_at_position(spot);
    KickFoulScoringTier::from_zone(zone)
}
