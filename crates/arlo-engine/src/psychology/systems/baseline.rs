use crate::spatial::decision_vector::extract_attribute_value;
use crate::weighting::{calculate_weighted_saturated_average, AttributeWeight};
use arlo_domain::sport_constants::{
    ATTRIBUTE_SATURATION_MULTIPLIER, ATTRIBUTE_SATURATION_THRESHOLD,
};
use arlo_domain::{AttributeKey, Player};
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