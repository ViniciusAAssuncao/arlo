use crate::ai::cognitive::RiskProfile;
use crate::ai::markov_decision::MarkovDecisionEvaluator;
use crate::artrine::constants::{SERIES_MAX_DOWNS, SERIES_TARGET_ADVANCE_MIRIM};
use crate::physical::PhysicalState;
use crate::spatial::proximity::calculate_distance_mirim;
use crate::world_state::GameStatePressure;
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player, Position, SlotRole};
use arlo_math::units::Position as VectorPosition;
use arlo_tactics::{DecisionEmphasis, PassingRange, PlayerInstructions};
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_decision_utilities(
    artrine: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    available_kinds: &[ArtrineDecisionKind],
    normalized_proximity: f64,
    drives_in_current_series: u32,
    remaining_downs: u8,
    pass_protection_net_advantage: f64,
    is_last_down: bool,
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
) -> Vec<(ArtrineDecisionKind, f64)> {
    let risk_profile = RiskProfile::from_player(artrine, attribute_keys, artrine_physical_state);
    let game_state_pressure = GameStatePressure::default();

    calculate_decision_utilities_with_context(
        artrine,
        attribute_keys,
        available_kinds,
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
    )
}

pub fn calculate_decision_utilities_with_context(
    artrine: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    available_kinds: &[ArtrineDecisionKind],
    normalized_proximity: f64,
    drives_in_current_series: u32,
    remaining_downs: u8,
    pass_protection_net_advantage: f64,
    is_last_down: bool,
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
) -> Vec<(ArtrineDecisionKind, f64)> {
    calculate_decision_utilities_with_context_and_free_path(
        artrine,
        attribute_keys,
        available_kinds,
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
        0.0,
    )
}

pub fn calculate_decision_utilities_with_context_and_free_path(
    artrine: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    available_kinds: &[ArtrineDecisionKind],
    normalized_proximity: f64,
    drives_in_current_series: u32,
    remaining_downs: u8,
    pass_protection_net_advantage: f64,
    is_last_down: bool,
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
    expected_free_path_mirim: f64,
) -> Vec<(ArtrineDecisionKind, f64)> {
    let down = SERIES_MAX_DOWNS.saturating_sub(remaining_downs).max(1);
    let remaining_advance_mirim = (SERIES_TARGET_ADVANCE_MIRIM - territory_advance_mirim).max(0.0);
    let distance_to_next_artro_mirim = calculate_distance_mirim(artrine_pos, next_artro_pos);

    MarkovDecisionEvaluator::evaluate_carrier_action_utilities(
        artrine,
        Position::Artrine,
        SlotRole::Standard,
        PlayerInstructions::default(),
        attribute_keys,
        available_kinds,
        normalized_proximity,
        drives_in_current_series,
        if is_last_down { SERIES_MAX_DOWNS } else { down },
        remaining_advance_mirim,
        pass_protection_net_advantage,
        best_available_target_weight,
        long_launch_target_weight,
        artrine_pos,
        pitch_control_ahead,
        distance_to_next_artro_mirim,
        pitch_length_mirim,
        85.0,
        offensive_gravity,
        passing_range,
        risk_profile,
        game_state_pressure,
        play_call_emphasis,
        artrine_physical_state,
        true,
        expected_free_path_mirim,
    )
}