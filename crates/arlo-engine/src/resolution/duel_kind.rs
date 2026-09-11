use arlo_domain::sport_constants::{
    ATTRIBUTE_SATURATION_THRESHOLD, DUEL_PHYSICALITY_BASELINE_ARTRO_BREAKTHROUGH,
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
        DuelKind::FinishingAttempt | DuelKind::ShortDistribution | DuelKind::FieldGoalAttempt => {
            2.7
        }
        DuelKind::LongDistribution | DuelKind::CrossDistribution | DuelKind::ArtroBreakthrough => {
            2.4
        }
        DuelKind::RouteContest | DuelKind::BallSecurityDistribution => 2.25,
        DuelKind::PassProtection | DuelKind::RunBreakthrough | DuelKind::AerialDuel => 2.0,
        DuelKind::LateralBlock => 1.9,
        DuelKind::CentralBlock | DuelKind::BallSecurityCarry => 1.8,
    };
    factor / ATTRIBUTE_SATURATION_THRESHOLD
}