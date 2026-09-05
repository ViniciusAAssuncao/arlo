use crate::resolution::duel_profiles::DuelProfile;
use crate::weighting::{apply_saturation, calculate_weighted_average};
use arlo_domain::sport_constants::{
    ATTRIBUTE_SATURATION_THRESHOLD, GROUP_AGGREGATION_SATURATION_MULTIPLIER,
    GROUP_AGGREGATION_SATURATION_THRESHOLD, GROUP_SATURATION_MULTIPLIER,
    GROUP_SATURATION_THRESHOLD,
};
use arlo_domain::{AttributeKey, Player};
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_player_duel_rating(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
) -> f64 {
    let mut items = Vec::new();
    for attr in player.attributes() {
        if let Some(key) = attribute_keys.get(&attr.attribute_definition_id()) {
            if let Some(w) = profile.weights().iter().find(|item| item.key == *key) {
                if w.weight > 0.0 {
                    items.push((attr.value() as f64, w.weight));
                }
            }
        }
    }
    calculate_weighted_average(&items).unwrap_or(0.0)
}

pub fn calculate_group_rating(ratings: &[f64]) -> f64 {
    if ratings.is_empty() {
        return 0.0;
    }
    let mut sorted = ratings.to_vec();
    sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    let lead = sorted[0];
    if sorted.len() == 1 {
        return lead;
    }

    let mut helper_sum = 0.0;
    for &rating in &sorted[1..] {
        let raw_contrib = (rating / ATTRIBUTE_SATURATION_THRESHOLD) * GROUP_SATURATION_THRESHOLD;
        let sat_contrib = apply_saturation(
            raw_contrib,
            GROUP_SATURATION_THRESHOLD,
            GROUP_SATURATION_MULTIPLIER,
        );
        helper_sum += sat_contrib;
    }

    let aggregated_bonus = apply_saturation(
        helper_sum,
        GROUP_AGGREGATION_SATURATION_THRESHOLD,
        GROUP_AGGREGATION_SATURATION_MULTIPLIER,
    );

    lead + aggregated_bonus
}

pub fn calculate_side_rating(
    players: &[&Player],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
) -> f64 {
    let ratings: Vec<f64> = players
        .iter()
        .map(|p| calculate_player_duel_rating(p, attribute_keys, profile))
        .collect();
    calculate_group_rating(&ratings)
}