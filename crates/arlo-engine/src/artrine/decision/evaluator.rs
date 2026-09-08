use crate::ai::cognitive::RiskProfile;
use crate::ai::markov_decision::MarkovDecisionEvaluator;
use crate::artrine::constants::{SERIES_MAX_DOWNS, SERIES_TARGET_ADVANCE_MIRIM};
use crate::physical::PhysicalState;
use crate::spatial::proximity::calculate_distance_mirim;
use crate::team_identity::TeamIdentityBias;
use crate::world_state::GameStatePressure;
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player};
use arlo_math::units::Position as VectorPosition;
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
    artrine_pos: VectorPosition,
    next_artro_pos: VectorPosition,
    pitch_control_ahead: f64,
    pitch_length_mirim: f64,
    offensive_gravity: f64,
    artrine_physical_state: &PhysicalState,
) -> Vec<(ArtrineDecisionKind, f64)> {
    let risk_profile = RiskProfile::from_player(artrine, attribute_keys, artrine_physical_state);
    let game_state_pressure = GameStatePressure::default();
    let team_identity_bias = TeamIdentityBias::default();

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
        artrine_pos,
        next_artro_pos,
        pitch_control_ahead,
        pitch_length_mirim,
        offensive_gravity,
        risk_profile,
        game_state_pressure,
        team_identity_bias,
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
    artrine_pos: VectorPosition,
    next_artro_pos: VectorPosition,
    pitch_control_ahead: f64,
    pitch_length_mirim: f64,
    offensive_gravity: f64,
    risk_profile: RiskProfile,
    game_state_pressure: GameStatePressure,
    team_identity_bias: TeamIdentityBias,
    artrine_physical_state: &PhysicalState,
) -> Vec<(ArtrineDecisionKind, f64)> {
    let down = SERIES_MAX_DOWNS.saturating_sub(remaining_downs).max(1);
    let remaining_advance_mirim = (SERIES_TARGET_ADVANCE_MIRIM - territory_advance_mirim).max(0.0);
    let distance_to_next_artro_mirim = calculate_distance_mirim(artrine_pos, next_artro_pos);

    MarkovDecisionEvaluator::evaluate_action_utilities_with_context(
        artrine,
        attribute_keys,
        available_kinds,
        normalized_proximity,
        drives_in_current_series,
        if is_last_down { SERIES_MAX_DOWNS } else { down },
        remaining_advance_mirim,
        pass_protection_net_advantage,
        best_available_target_weight,
        pitch_control_ahead,
        distance_to_next_artro_mirim,
        pitch_length_mirim,
        offensive_gravity,
        risk_profile,
        game_state_pressure,
        team_identity_bias,
        artrine_physical_state,
    )
}
