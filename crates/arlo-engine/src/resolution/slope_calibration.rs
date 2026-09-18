use crate::resolution::duel_kind::DuelKind;
use arlo_domain::sport_constants::{
    ATTRIBUTE_SATURATION_THRESHOLD,
    KICK_BLOCK_ATTEMPT_LOGISTIC_FACTOR,
};

pub fn logistic_slope_for(kind: DuelKind) -> f64 {
    let factor = match kind {
        DuelKind::CentralBlock => 28.0,
        DuelKind::ArtroBreakthrough => 28.0,
        DuelKind::PassProtection => 26.0,
        DuelKind::RunBreakthrough => 26.0,
        DuelKind::LateralBlock => 25.0,
        DuelKind::AerialDuel => 25.0,
        DuelKind::BallSecurityCarry => 24.0,
        DuelKind::FinishingAttempt => 22.0,
        DuelKind::ShortDistribution => 22.0,
        DuelKind::FieldGoalAttempt => 20.0,
        DuelKind::LongDistribution => 20.0,
        DuelKind::CrossDistribution => 20.0,
        DuelKind::RouteContest => 18.0,
        DuelKind::BallSecurityDistribution => 18.0,
        DuelKind::KickBlockAttempt => {
            KICK_BLOCK_ATTEMPT_LOGISTIC_FACTOR * ATTRIBUTE_SATURATION_THRESHOLD * 3.5
        }
    };
    factor / ATTRIBUTE_SATURATION_THRESHOLD
}
