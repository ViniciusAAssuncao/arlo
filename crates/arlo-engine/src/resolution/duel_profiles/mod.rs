pub use crate::attributes::profiles::*;
pub use crate::caching::duel_profile_cache::get_cached_duel_profiles;

use crate::resolution::duel_kind::DuelKind;
use crate::weighting::AttributeWeight;

pub type DuelProfile = crate::attributes::profiles::AttributeProfile;

pub fn get_duel_profiles(kind: DuelKind) -> &'static (DuelProfile, DuelProfile) {
    get_cached_duel_profiles(kind)
}

pub fn get_duel_profile_weights(
    kind: DuelKind,
) -> (&'static [AttributeWeight], &'static [AttributeWeight]) {
    let (att, def) = get_cached_duel_profiles(kind);
    (att.weights(), def.weights())
}