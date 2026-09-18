use arlo_domain::sport_constants::{
    ATTRIBUTE_SATURATION_THRESHOLD, DUEL_PHYSICALITY_BASELINE_ARTRO_BREAKTHROUGH,
    DUEL_PHYSICALITY_BASELINE_BALL_SECURITY_CARRY,
    DUEL_PHYSICALITY_BASELINE_BALL_SECURITY_DISTRIBUTION,
    DUEL_PHYSICALITY_BASELINE_CENTRAL_BLOCK, DUEL_PHYSICALITY_BASELINE_LATERAL_BLOCK,
    KICK_BLOCK_ATTEMPT_LOGISTIC_FACTOR,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DuelKind {
    PassProtection,
    RouteContest,
    RunBreakthrough,
    CentralBlock,
    LateralBlock,
    ArtroBreakthrough,
    AerialDuel,
    FinishingAttempt,
    FieldGoalAttempt,
    ShortDistribution,
    LongDistribution,
    CrossDistribution,
    BallSecurityCarry,
    BallSecurityDistribution,
    KickBlockAttempt,
}

impl DuelKind {
    pub fn logistic_slope(self) -> f64 {
        logistic_slope_for(self)
    }

    pub fn is_contact_duel(&self) -> bool {
        matches!(
            self,
            DuelKind::ArtroBreakthrough
                | DuelKind::CentralBlock
                | DuelKind::LateralBlock
                | DuelKind::BallSecurityCarry
                | DuelKind::BallSecurityDistribution
        )
    }

    pub fn physicality_baseline(&self) -> f64 {
        match self {
            DuelKind::ArtroBreakthrough => DUEL_PHYSICALITY_BASELINE_ARTRO_BREAKTHROUGH,
            DuelKind::CentralBlock => DUEL_PHYSICALITY_BASELINE_CENTRAL_BLOCK,
            DuelKind::LateralBlock => DUEL_PHYSICALITY_BASELINE_LATERAL_BLOCK,
            DuelKind::BallSecurityCarry => DUEL_PHYSICALITY_BASELINE_BALL_SECURITY_CARRY,
            DuelKind::BallSecurityDistribution => {
                DUEL_PHYSICALITY_BASELINE_BALL_SECURITY_DISTRIBUTION
            }
            _ => 0.0,
        }
    }
}

pub fn logistic_slope_for(kind: DuelKind) -> f64 {
    let factor = match kind {
        DuelKind::FinishingAttempt => 12.0,
        DuelKind::ArtroBreakthrough => 11.5,
        DuelKind::ShortDistribution => 11.0,
        DuelKind::LongDistribution | DuelKind::CrossDistribution => 10.5,
        DuelKind::FieldGoalAttempt => 10.0,
        DuelKind::RouteContest | DuelKind::BallSecurityDistribution => 9.2,
        DuelKind::PassProtection | DuelKind::RunBreakthrough | DuelKind::AerialDuel => 8.8,
        DuelKind::LateralBlock => 8.2,
        DuelKind::CentralBlock | DuelKind::BallSecurityCarry => 8.0,
        DuelKind::KickBlockAttempt => {
            KICK_BLOCK_ATTEMPT_LOGISTIC_FACTOR * ATTRIBUTE_SATURATION_THRESHOLD * 1.8
        }
    };
    factor / ATTRIBUTE_SATURATION_THRESHOLD
}