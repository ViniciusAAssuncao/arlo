use crate::ai::cognitive::RiskProfile;
use crate::artrine::decision::available_decisions::available_decision_kinds;
use crate::artrine::decision::evaluator::calculate_decision_utilities_with_context;
use crate::artrine::decision::sampler::sample_artrine_decision;
use crate::physical::PhysicalState;
use crate::psychology::state::ImpulseState;
use crate::psychology::systems::baseline::calculate_player_impulse_baseline;
use crate::world_state::GameStatePressure;
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player};
use arlo_math::units::Position as VectorPosition;
use arlo_math::Probability;
use arlo_tactics::{DecisionEmphasis, PassingRange};
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
    long_launch_target_weight: f64,
    artrine_pos: VectorPosition,
    next_artro_pos: VectorPosition,
    pitch_control_ahead: f64,
    pitch_length_mirim: f64,
    offensive_gravity: f64,
    passing_range: PassingRange,
    play_call_emphasis: DecisionEmphasis,
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
        long_launch_target_weight,
        artrine_pos,
        next_artro_pos,
        pitch_control_ahead,
        pitch_length_mirim,
        offensive_gravity,
        passing_range,
        play_call_emphasis,
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
    long_launch_target_weight: f64,
    artrine_pos: VectorPosition,
    next_artro_pos: VectorPosition,
    pitch_control_ahead: f64,
    pitch_length_mirim: f64,
    offensive_gravity: f64,
    passing_range: PassingRange,
    play_call_emphasis: DecisionEmphasis,
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
        long_launch_target_weight,
        artrine_pos,
        next_artro_pos,
        pitch_control_ahead,
        pitch_length_mirim,
        offensive_gravity,
        passing_range,
        risk_profile,
        game_state_pressure,
        play_call_emphasis,
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
    long_launch_target_weight: f64,
    artrine_pos: VectorPosition,
    next_artro_pos: VectorPosition,
    pitch_control_ahead: f64,
    pitch_length_mirim: f64,
    offensive_gravity: f64,
    passing_range: PassingRange,
    risk_profile: RiskProfile,
    game_state_pressure: GameStatePressure,
    play_call_emphasis: DecisionEmphasis,
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
        long_launch_target_weight,
        artrine_pos,
        next_artro_pos,
        pitch_control_ahead,
        pitch_length_mirim,
        offensive_gravity,
        passing_range,
        risk_profile,
        game_state_pressure,
        play_call_emphasis,
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
    long_launch_target_weight: f64,
    artrine_pos: VectorPosition,
    next_artro_pos: VectorPosition,
    pitch_control_ahead: f64,
    pitch_length_mirim: f64,
    offensive_gravity: f64,
    passing_range: PassingRange,
    risk_profile: RiskProfile,
    game_state_pressure: GameStatePressure,
    play_call_emphasis: DecisionEmphasis,
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
        long_launch_target_weight,
        artrine_pos,
        next_artro_pos,
        pitch_control_ahead,
        pitch_length_mirim,
        offensive_gravity,
        passing_range,
        risk_profile,
        game_state_pressure,
        play_call_emphasis,
        artrine_physical_state,
    );

    let result = sample_artrine_decision(
        artrine,
        attribute_keys,
        &utilities,
        artrine_physical_state,
        artrine_impulse_state,
        rng,
    );

    crate::psychology::systems::instrumentation::instrument_artrine_decision(artrine.id(), &result);

    result
}
