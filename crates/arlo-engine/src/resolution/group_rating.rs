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

pub fn calculate_anchored_rating(anchor_rating: f64, helper_ratings: &[f64]) -> f64 {
    if helper_ratings.is_empty() {
        return anchor_rating;
    }

    let mut helper_sum = 0.0;
    for &rating in helper_ratings {
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

    anchor_rating + aggregated_bonus
}

pub fn calculate_anchored_side_rating(
    anchor: &Player,
    helpers: &[&Player],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
) -> f64 {
    let anchor_rating = calculate_player_duel_rating(anchor, attribute_keys, profile);
    let helper_ratings: Vec<f64> = helpers
        .iter()
        .map(|p| calculate_player_duel_rating(p, attribute_keys, profile))
        .collect();
    calculate_anchored_rating(anchor_rating, &helper_ratings)
}

pub fn identify_lead_player<'a>(
    players: &[&'a Player],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &DuelProfile,
) -> Option<&'a Player> {
    players.iter().copied().max_by(|a, b| {
        let rating_a = calculate_player_duel_rating(a, attribute_keys, profile);
        let rating_b = calculate_player_duel_rating(b, attribute_keys, profile);
        rating_a
            .partial_cmp(&rating_b)
            .unwrap_or(std::cmp::Ordering::Equal)
    })
}