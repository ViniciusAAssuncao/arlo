use crate::current_ability::profiles::get_profile_for_position;
use crate::current_ability::weights::PositionWeightProfile;
use arlo_domain::{AttributeKey, Position};
use std::collections::HashMap;

pub fn calculate_profile_similarity(
    profile_a: &PositionWeightProfile,
    profile_b: &PositionWeightProfile,
) -> f64 {
    let mut weights_a: HashMap<AttributeKey, f64> = HashMap::with_capacity(profile_a.weights.len());
    for w in &profile_a.weights {
        weights_a.insert(w.key, w.weight);
    }

    let mut dot_product = 0.0;
    let mut mag_b_sq = 0.0;

    for w in &profile_b.weights {
        mag_b_sq += w.weight * w.weight;
        if let Some(&wa) = weights_a.get(&w.key) {
            dot_product += wa * w.weight;
        }
    }

    let mag_a_sq: f64 = profile_a.weights.iter().map(|w| w.weight * w.weight).sum();

    if mag_a_sq <= 0.0 || mag_b_sq <= 0.0 {
        return 0.0;
    }

    let mag_a = mag_a_sq.sqrt();
    let mag_b = mag_b_sq.sqrt();

    (dot_product / (mag_a * mag_b)).clamp(0.0, 1.0)
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
    crate::lineup_runtime::position_profile_cache::get_position_similarity(a, b)
}