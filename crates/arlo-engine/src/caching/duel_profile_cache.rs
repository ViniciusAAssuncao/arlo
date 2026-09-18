use crate::attributes::profiles::{get_duel_attribute_profiles, AttributeProfile as DuelProfile};
use crate::resolution::duel_kind::DuelKind;
use std::collections::HashMap;
use std::sync::OnceLock;

static DUEL_PROFILES_CACHE: OnceLock<HashMap<DuelKind, (DuelProfile, DuelProfile)>> =
    OnceLock::new();

fn build_duel_profiles(kind: DuelKind) -> (DuelProfile, DuelProfile) {
    get_duel_attribute_profiles(kind)
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

pub fn get_duel_profiles(kind: DuelKind) -> (DuelProfile, DuelProfile) {
    get_duel_attribute_profiles(kind)
}