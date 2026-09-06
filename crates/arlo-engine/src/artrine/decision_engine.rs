use crate::artrine::utility::{available_decision_kinds, calculate_decision_utilities};
use crate::spatial::decision_vector::extract_attribute_value;
use arlo_domain::sport_constants::decision_steepness_for;
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player};
use arlo_math::stats::categorical::sample_categorical;
use arlo_math::stats::contrast::softmax_weights;
use arlo_math::Probability;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ArtrineDecisionResult {
    pub chosen: ArtrineDecisionKind,
    pub chosen_probability: Probability,
}

impl ArtrineDecisionResult {
    pub fn new(chosen: ArtrineDecisionKind, chosen_probability: Probability) -> Self {
        Self {
            chosen,
            chosen_probability,
        }
    }

    pub fn chosen(&self) -> ArtrineDecisionKind {
        self.chosen
    }

    pub fn chosen_probability(&self) -> Probability {
        self.chosen_probability
    }
}

pub fn resolve_artrine_decision<R: Rng + ?Sized>(
    artrine: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    normalized_proximity: f64,
    drives_in_current_series: u32,
    remaining_downs: u8,
    pass_protection_net_advantage: f64,
    is_last_down: bool,
    territory_advance_mirim: f64,
    best_available_target_weight: f64,
    rng: &mut R,
) -> ArtrineDecisionResult {
    let available_kinds =
        available_decision_kinds(drives_in_current_series, territory_advance_mirim, is_last_down);

    let utilities = calculate_decision_utilities(
        artrine,
        attribute_keys,
        &available_kinds,
        normalized_proximity,
        drives_in_current_series,
        remaining_downs,
        pass_protection_net_advantage,
        is_last_down,
        territory_advance_mirim,
        best_available_target_weight,
    );

    if utilities.is_empty() {
        return ArtrineDecisionResult::new(
            ArtrineDecisionKind::SelfCarry,
            Probability::new_clamped(1.0),
        );
    }

    let raw_utilities: Vec<f64> = utilities.iter().map(|(_, u)| *u).collect();
    let decisions_val = extract_attribute_value(artrine, attribute_keys, AttributeKey::Decisions);
    let steepness = decision_steepness_for(decisions_val);
    let weights = softmax_weights(&raw_utilities, steepness);
    let total_weight: f64 = weights.iter().sum();

    let selected_index = sample_categorical(&weights, rng).unwrap_or(0);
    let (chosen_kind, _) = utilities[selected_index];

    let prob_value = if total_weight > 0.0 {
        weights[selected_index] / total_weight
    } else {
        1.0 / (weights.len() as f64)
    };

    let chosen_probability = Probability::new_clamped(prob_value);

    ArtrineDecisionResult::new(chosen_kind, chosen_probability)
}