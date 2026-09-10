use crate::attributes::PlayerAttributeTable;
use crate::physical::systems::degradation::extract_effective_attribute_value_with_impulse;
use crate::physical::PhysicalState;
use crate::psychology::state::ImpulseState;
use crate::psychology::systems::baseline::calculate_player_impulse_baseline_from_table_with_profile;
use arlo_domain::sport_constants::decision_steepness_with_impulse;
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player};
use arlo_math::stats::categorical::sample_categorical;
use arlo_math::stats::contrast::softmax_weights;
use arlo_math::Probability;
use rand::Rng;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CarrierDecisionResult {
    pub chosen: ArtrineDecisionKind,
    pub chosen_probability: Probability,
}

impl CarrierDecisionResult {
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

pub fn sample_carrier_decision_from_table<R: Rng + ?Sized>(
    _carrier: &Player,
    table: &PlayerAttributeTable,
    utilities: &[(ArtrineDecisionKind, f64)],
    carrier_physical_state: &PhysicalState,
    carrier_impulse_state: &ImpulseState,
    rng: &mut R,
) -> CarrierDecisionResult {
    if utilities.is_empty() {
        return CarrierDecisionResult::new(
            ArtrineDecisionKind::SelfCarry,
            Probability::new_clamped(1.0),
        );
    }

    let raw_utilities: SmallVec<[f64; 5]> = utilities.iter().map(|(_, u)| *u).collect();
    let profile = crate::caching::impulse_baseline_profile();
    let baseline = calculate_player_impulse_baseline_from_table_with_profile(table, profile);
    let decisions_val = extract_effective_attribute_value_with_impulse(
        table,
        AttributeKey::Decisions,
        carrier_physical_state,
        carrier_impulse_state,
        baseline,
    );
    let steepness = decision_steepness_with_impulse(decisions_val, carrier_impulse_state.value());
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
    CarrierDecisionResult::new(chosen_kind, chosen_probability)
}

pub fn sample_carrier_decision<R: Rng + ?Sized>(
    carrier: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    utilities: &[(ArtrineDecisionKind, f64)],
    carrier_physical_state: &PhysicalState,
    carrier_impulse_state: &ImpulseState,
    rng: &mut R,
) -> CarrierDecisionResult {
    let table = PlayerAttributeTable::from_player(carrier, attribute_keys);
    sample_carrier_decision_from_table(
        carrier,
        &table,
        utilities,
        carrier_physical_state,
        carrier_impulse_state,
        rng,
    )
}