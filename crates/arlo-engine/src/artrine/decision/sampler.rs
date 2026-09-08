use crate::physical::systems::degradation::extract_effective_attribute_value_with_impulse;
use crate::physical::PhysicalState;
use crate::psychology::state::ImpulseState;
use arlo_domain::sport_constants::decision_steepness_with_impulse;
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player};
use arlo_math::stats::categorical::sample_categorical;
use arlo_math::stats::contrast::softmax_weights;
use arlo_math::Probability;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

use super::decision_engine::ArtrineDecisionResult;

pub fn sample_artrine_decision<R: Rng + ?Sized>(
    artrine: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    utilities: &[(ArtrineDecisionKind, f64)],
    artrine_physical_state: &PhysicalState,
    artrine_impulse_state: &ImpulseState,
    rng: &mut R,
) -> ArtrineDecisionResult {
    if utilities.is_empty() {
        return ArtrineDecisionResult::new(
            ArtrineDecisionKind::SelfCarry,
            Probability::new_clamped(1.0),
        );
    }

    let raw_utilities: Vec<f64> = utilities.iter().map(|(_, u)| *u).collect();
    let decisions_val = extract_effective_attribute_value_with_impulse(
        artrine,
        attribute_keys,
        AttributeKey::Decisions,
        artrine_physical_state,
        artrine_impulse_state,
    );
    let steepness = decision_steepness_with_impulse(decisions_val, artrine_impulse_state.value());
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