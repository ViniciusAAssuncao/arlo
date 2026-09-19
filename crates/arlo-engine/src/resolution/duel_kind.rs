use crate::resolution::slope_calibration::logistic_slope_for;
use arlo_domain::sport_constants::{
    DUEL_PHYSICALITY_BASELINE_ARTRO_BREAKTHROUGH,
    DUEL_PHYSICALITY_BASELINE_BALL_SECURITY_CARRY,
    DUEL_PHYSICALITY_BASELINE_BALL_SECURITY_DISTRIBUTION,
    DUEL_PHYSICALITY_BASELINE_CENTRAL_BLOCK, DUEL_PHYSICALITY_BASELINE_LATERAL_BLOCK,
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

pub use crate::resolution::slope_calibration::logistic_slope_for as duel_logistic_slope_for;