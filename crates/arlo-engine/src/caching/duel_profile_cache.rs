use crate::resolution::duel_kind::DuelKind;
use crate::resolution::duel_profiles::{
    defense_duels, goalkeeping_duels, offense_duels, DuelProfile,
};
use std::collections::HashMap;
use std::sync::OnceLock;

static DUEL_PROFILES_CACHE: OnceLock<HashMap<DuelKind, (DuelProfile, DuelProfile)>> =
    OnceLock::new();

fn build_duel_profiles(kind: DuelKind) -> (DuelProfile, DuelProfile) {
    match kind {
        DuelKind::PassProtection | DuelKind::KickBlockAttempt => (
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
        DuelKind::FieldGoalAttempt => (
            offense_duels::field_goal_profile(),
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

fn init_duel_profiles_cache() -> HashMap<DuelKind, (DuelProfile, DuelProfile)> {
    let kinds = [
        DuelKind::PassProtection,
        DuelKind::RouteContest,
        DuelKind::RunBreakthrough,
        DuelKind::CentralBlock,
        DuelKind::LateralBlock,
        DuelKind::ArtroBreakthrough,
        DuelKind::AerialDuel,
        DuelKind::FinishingAttempt,
        DuelKind::FieldGoalAttempt,
        DuelKind::ShortDistribution,
        DuelKind::LongDistribution,
        DuelKind::CrossDistribution,
        DuelKind::BallSecurityCarry,
        DuelKind::BallSecurityDistribution,
        DuelKind::KickBlockAttempt,
    ];
    let mut map = HashMap::with_capacity(kinds.len());
    for kind in kinds {
        map.insert(kind, build_duel_profiles(kind));
    }
    map
}

pub fn get_cached_duel_profiles(kind: DuelKind) -> &'static (DuelProfile, DuelProfile) {
    &DUEL_PROFILES_CACHE.get_or_init(init_duel_profiles_cache)[&kind]
}