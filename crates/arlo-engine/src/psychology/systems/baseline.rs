use crate::spatial::decision_vector::extract_attribute_value;
use crate::weighting::{calculate_weighted_saturated_average, AttributeWeight};
use arlo_domain::sport_constants::{
    ATTRIBUTE_SATURATION_MULTIPLIER, ATTRIBUTE_SATURATION_THRESHOLD, HOME_IMPULSE_BASELINE_BOOST,
    MAX_CAPTAINCY_BASELINE_BOOST,
};
use arlo_domain::{AttributeKey, CaptaincyRole, Player};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImpulseBaselineProfile {
    weights: Vec<AttributeWeight>,
}

impl ImpulseBaselineProfile {
    pub fn new(weights: Vec<AttributeWeight>) -> Self {
        Self { weights }
    }

    pub fn weights(&self) -> &[AttributeWeight] {
        &self.weights
    }
}

pub fn default_impulse_baseline_profile() -> ImpulseBaselineProfile {
    ImpulseBaselineProfile::new(vec![
        AttributeWeight::new(AttributeKey::Determination, 5.0),
        AttributeWeight::new(AttributeKey::Composure, 4.5),
        AttributeWeight::new(AttributeKey::Bravery, 4.0),
        AttributeWeight::new(AttributeKey::Consistency, 4.0),
        AttributeWeight::new(AttributeKey::Concentration, 3.5),
        AttributeWeight::new(AttributeKey::Leadership, 3.0),
        AttributeWeight::new(AttributeKey::Teamwork, 2.5),
    ])
}

pub fn calculate_player_impulse_baseline_with_profile(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    profile: &ImpulseBaselineProfile,
) -> f64 {
    let mut items = Vec::with_capacity(profile.weights().len());
    for w in profile.weights() {
        if w.weight > 0.0 {
            let val = extract_attribute_value(player, attribute_keys, w.key);
            items.push((val, w.weight));
        }
    }

    let avg = calculate_weighted_saturated_average(
        &items,
        ATTRIBUTE_SATURATION_THRESHOLD,
        ATTRIBUTE_SATURATION_MULTIPLIER,
    )
    .unwrap_or(10.0);

    let norm = (avg - 10.0) / 10.0;
    let mapped = 100.0 / (1.0 + (-1.8 * norm).exp());
    mapped.clamp(0.0, 100.0)
}

pub fn calculate_player_impulse_baseline(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let profile = default_impulse_baseline_profile();
    calculate_player_impulse_baseline_with_profile(player, attribute_keys, &profile)
}

pub fn find_active_captain<'a>(
    players: &[&'a Player],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> Option<&'a Player> {
    if players.is_empty() {
        return None;
    }

    if let Some(&captain) = players
        .iter()
        .find(|p| p.captaincy_role() == Some(CaptaincyRole::Captain))
    {
        return Some(captain);
    }

    if let Some(&vice_captain) = players
        .iter()
        .find(|p| p.captaincy_role() == Some(CaptaincyRole::ViceCaptain))
    {
        return Some(vice_captain);
    }

    players.iter().copied().max_by(|a, b| {
        let lead_a = extract_attribute_value(a, attribute_keys, AttributeKey::Leadership);
        let lead_b = extract_attribute_value(b, attribute_keys, AttributeKey::Leadership);
        lead_a.partial_cmp(&lead_b).unwrap_or(std::cmp::Ordering::Equal)
    })
}

pub fn calculate_captaincy_influence(
    captain: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let leadership = extract_attribute_value(captain, attribute_keys, AttributeKey::Leadership);
    let communication = extract_attribute_value(captain, attribute_keys, AttributeKey::Communication);
    let determination = extract_attribute_value(captain, attribute_keys, AttributeKey::Determination);
    let teamwork = extract_attribute_value(captain, attribute_keys, AttributeKey::Teamwork);

    let composite = (leadership * 0.40
        + communication * 0.25
        + determination * 0.20
        + teamwork * 0.15)
        / 20.0;

    let delta = (composite.clamp(0.0, 1.0) - 0.50) / 0.50;
    delta.clamp(-1.0, 1.0)
}

pub fn calculate_player_contextual_baseline(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    captain: Option<&Player>,
    is_home: bool,
) -> f64 {
    let base = calculate_player_impulse_baseline(player, attribute_keys);

    let captain_boost = match captain {
        Some(cap) if cap.id() == player.id() => {
            let influence = calculate_captaincy_influence(cap, attribute_keys);
            influence * MAX_CAPTAINCY_BASELINE_BOOST * 0.50
        }
        Some(cap) => {
            let influence = calculate_captaincy_influence(cap, attribute_keys);
            influence * MAX_CAPTAINCY_BASELINE_BOOST
        }
        None => 0.0,
    };

    let home_boost = if is_home {
        HOME_IMPULSE_BASELINE_BOOST
    } else {
        0.0
    };

    (base + captain_boost + home_boost).clamp(5.0, 98.0)
}