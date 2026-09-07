use crate::ai::cognitive::RiskProfile;
use crate::artrine::utility::{
    available_decision_kinds, calculate_decision_utilities_with_context,
};
use crate::physical::systems::degradation::extract_effective_attribute_value_with_impulse;
use crate::physical::PhysicalState;
use crate::psychology::state::ImpulseState;
use crate::psychology::systems::baseline::calculate_player_impulse_baseline;
use crate::world_state::GameStatePressure;
use arlo_domain::sport_constants::decision_steepness_with_impulse;
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player};
use arlo_math::stats::categorical::sample_categorical;
use arlo_math::stats::contrast::softmax_weights;
use arlo_math::units::Position as VectorPosition;
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
    is_bonus_phase: bool,
    territory_advance_mirim: f64,
    best_available_target_weight: f64,
    artrine_pos: VectorPosition,
    next_artro_pos: VectorPosition,
    pitch_control_ahead: f64,
    pitch_length_mirim: f64,
    offensive_gravity: f64,
    artrine_physical_state: &PhysicalState,
    rng: &mut R,
) -> ArtrineDecisionResult {
    let baseline = calculate_player_impulse_baseline(artrine, attribute_keys);
    let impulse_state = ImpulseState::from_baseline(baseline);
    resolve_artrine_decision_with_impulse(
        artrine,
        attribute_keys,
        normalized_proximity,
        drives_in_current_series,
        remaining_downs,
        pass_protection_net_advantage,
        is_last_down,
        is_bonus_phase,
        territory_advance_mirim,
        best_available_target_weight,
        artrine_pos,
        next_artro_pos,
        pitch_control_ahead,
        pitch_length_mirim,
        offensive_gravity,
        artrine_physical_state,
        &impulse_state,
        rng,
    )
}

pub fn resolve_artrine_decision_with_impulse<R: Rng + ?Sized>(
    artrine: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    normalized_proximity: f64,
    drives_in_current_series: u32,
    remaining_downs: u8,
    pass_protection_net_advantage: f64,
    is_last_down: bool,
    is_bonus_phase: bool,
    territory_advance_mirim: f64,
    best_available_target_weight: f64,
    artrine_pos: VectorPosition,
    next_artro_pos: VectorPosition,
    pitch_control_ahead: f64,
    pitch_length_mirim: f64,
    offensive_gravity: f64,
    artrine_physical_state: &PhysicalState,
    artrine_impulse_state: &ImpulseState,
    rng: &mut R,
) -> ArtrineDecisionResult {
    let risk_profile = RiskProfile::from_player_with_impulse(
        artrine,
        attribute_keys,
        artrine_physical_state,
        artrine_impulse_state,
    );
    let game_state_pressure = GameStatePressure::default();

    resolve_artrine_decision_with_context_and_impulse(
        artrine,
        attribute_keys,
        normalized_proximity,
        drives_in_current_series,
        remaining_downs,
        pass_protection_net_advantage,
        is_last_down,
        is_bonus_phase,
        territory_advance_mirim,
        best_available_target_weight,
        artrine_pos,
        next_artro_pos,
        pitch_control_ahead,
        pitch_length_mirim,
        offensive_gravity,
        risk_profile,
        game_state_pressure,
        artrine_physical_state,
        artrine_impulse_state,
        rng,
    )
}

pub fn resolve_artrine_decision_with_context<R: Rng + ?Sized>(
    artrine: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    normalized_proximity: f64,
    drives_in_current_series: u32,
    remaining_downs: u8,
    pass_protection_net_advantage: f64,
    is_last_down: bool,
    is_bonus_phase: bool,
    territory_advance_mirim: f64,
    best_available_target_weight: f64,
    artrine_pos: VectorPosition,
    next_artro_pos: VectorPosition,
    pitch_control_ahead: f64,
    pitch_length_mirim: f64,
    offensive_gravity: f64,
    risk_profile: RiskProfile,
    game_state_pressure: GameStatePressure,
    artrine_physical_state: &PhysicalState,
    rng: &mut R,
) -> ArtrineDecisionResult {
    let baseline = calculate_player_impulse_baseline(artrine, attribute_keys);
    let impulse_state = ImpulseState::from_baseline(baseline);
    resolve_artrine_decision_with_context_and_impulse(
        artrine,
        attribute_keys,
        normalized_proximity,
        drives_in_current_series,
        remaining_downs,
        pass_protection_net_advantage,
        is_last_down,
        is_bonus_phase,
        territory_advance_mirim,
        best_available_target_weight,
        artrine_pos,
        next_artro_pos,
        pitch_control_ahead,
        pitch_length_mirim,
        offensive_gravity,
        risk_profile,
        game_state_pressure,
        artrine_physical_state,
        &impulse_state,
        rng,
    )
}

pub fn resolve_artrine_decision_with_context_and_impulse<R: Rng + ?Sized>(
    artrine: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    normalized_proximity: f64,
    drives_in_current_series: u32,
    remaining_downs: u8,
    pass_protection_net_advantage: f64,
    is_last_down: bool,
    is_bonus_phase: bool,
    territory_advance_mirim: f64,
    best_available_target_weight: f64,
    artrine_pos: VectorPosition,
    next_artro_pos: VectorPosition,
    pitch_control_ahead: f64,
    pitch_length_mirim: f64,
    offensive_gravity: f64,
    risk_profile: RiskProfile,
    game_state_pressure: GameStatePressure,
    artrine_physical_state: &PhysicalState,
    artrine_impulse_state: &ImpulseState,
    rng: &mut R,
) -> ArtrineDecisionResult {
    let available_kinds = available_decision_kinds(
        drives_in_current_series,
        territory_advance_mirim,
        is_last_down,
        is_bonus_phase,
    );

    let utilities = calculate_decision_utilities_with_context(
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
        artrine_pos,
        next_artro_pos,
        pitch_control_ahead,
        pitch_length_mirim,
        offensive_gravity,
        risk_profile,
        game_state_pressure,
        artrine_physical_state,
    );

    if utilities.is_empty() {
        let default_result = ArtrineDecisionResult::new(
            ArtrineDecisionKind::SelfCarry,
            Probability::new_clamped(1.0),
        );
        crate::psychology::systems::instrumentation::instrument_artrine_decision(
            artrine.id(),
            &default_result,
        );
        return default_result;
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
    let result = ArtrineDecisionResult::new(chosen_kind, chosen_probability);

    crate::psychology::systems::instrumentation::instrument_artrine_decision(
        artrine.id(),
        &result,
    );

    result
}