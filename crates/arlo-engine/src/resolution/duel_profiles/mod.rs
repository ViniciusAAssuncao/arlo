pub mod defense_duels;
pub mod goalkeeping_duels;
pub mod offense_duels;

pub use defense_duels::*;
pub use goalkeeping_duels::*;
pub use offense_duels::*;

use crate::resolution::duel_kind::DuelKind;
use crate::weighting::AttributeWeight;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DuelProfile {
    weights: Vec<AttributeWeight>,
}

impl DuelProfile {
    pub fn new(weights: Vec<AttributeWeight>) -> Self {
        Self { weights }
    }

    pub fn weights(&self) -> &[AttributeWeight] {
        &self.weights
    }
}

pub fn get_duel_profiles(kind: DuelKind) -> (DuelProfile, DuelProfile) {
    match kind {
        DuelKind::PassProtection => (
            offense_duels::pass_protection_profile(),
            defense_duels::pass_rush_profile(),
        ),
        DuelKind::RouteContest => (
            offense_duels::route_contest_profile(),
            defense_duels::coverage_profile(),
        ),
        DuelKind::RunBreakthrough => (
            offense_duels::run_breakthrough_profile(),
            defense_duels::run_containment_profile(),
        ),
        DuelKind::CentralBlock => (
            offense_duels::central_block_profile(),
            defense_duels::central_resistance_profile(),
        ),
        DuelKind::LateralBlock => (
            offense_duels::lateral_block_profile(),
            defense_duels::lateral_resistance_profile(),
        ),
        DuelKind::ArtroBreakthrough => (
            offense_duels::artro_breakthrough_profile(),
            defense_duels::artro_defense_profile(),
        ),
        DuelKind::AerialDuel => (
            offense_duels::aerial_duel_profile(),
            defense_duels::aerial_defense_profile(),
        ),
        DuelKind::FinishingAttempt => (
            offense_duels::finishing_attempt_profile(),
            goalkeeping_duels::shot_stopping_profile(),
        ),
        DuelKind::ShortDistribution => (
            offense_duels::short_distribution_profile(),
            defense_duels::coverage_profile(),
        ),
        DuelKind::LongDistribution => (
            offense_duels::long_distribution_profile(),
            defense_duels::aerial_defense_profile(),
        ),
        DuelKind::CrossDistribution => (
            offense_duels::cross_distribution_profile(),
            defense_duels::coverage_profile(),
        ),
        DuelKind::BallSecurityCarry => (
            offense_duels::ball_security_carry_profile(),
            defense_duels::dispossession_profile(),
        ),
        DuelKind::BallSecurityDistribution => (
            offense_duels::ball_security_distribution_profile(),
            defense_duels::dispossession_profile(),
        ),
    }
}