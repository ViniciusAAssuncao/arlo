use crate::caching::position_profile_cache::get_position_similarity;
use crate::current_ability::profiles::get_profile_for_position;
use crate::current_ability::weights::PositionWeightProfile;
use arlo_domain::{AttributeKey, Position};
use arlo_math::stats::cosine_similarity;
use std::collections::{HashMap, HashSet};

pub fn calculate_profile_similarity(
    profile_a: &PositionWeightProfile,
    profile_b: &PositionWeightProfile,
) -> f64 {
    let mut weights_a: HashMap<AttributeKey, f64> = HashMap::with_capacity(profile_a.weights.len());
    for w in &profile_a.weights {
        weights_a.insert(w.key, w.weight);
    }

    let mut weights_b: HashMap<AttributeKey, f64> = HashMap::with_capacity(profile_b.weights.len());
    for w in &profile_b.weights {
        weights_b.insert(w.key, w.weight);
    }

    let mut all_keys: HashSet<AttributeKey> =
        HashSet::with_capacity(weights_a.len() + weights_b.len());
    all_keys.extend(weights_a.keys());
    all_keys.extend(weights_b.keys());

    let mut vec_a = Vec::with_capacity(all_keys.len());
    let mut vec_b = Vec::with_capacity(all_keys.len());

    for key in all_keys {
        vec_a.push(*weights_a.get(&key).unwrap_or(&0.0));
        vec_b.push(*weights_b.get(&key).unwrap_or(&0.0));
    }

    cosine_similarity(&vec_a, &vec_b).clamp(0.0, 1.0)
}

pub fn calculate_position_similarity(a: Position, b: Position) -> f64 {
    if a == b {
        return 1.0;
    }
    let profile_a = get_profile_for_position(a);
    let profile_b = get_profile_for_position(b);
    calculate_profile_similarity(&profile_a, &profile_b)
}

pub fn position_similarity(a: Position, b: Position) -> f64 {
    get_position_similarity(a, b)
}
