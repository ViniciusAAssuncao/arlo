use crate::ai::cognitive::RiskProfile;
use crate::ai::epv::DynamicEpvModel;
use crate::ai::evaluators::DecisionEvaluationContext;
use crate::artrine::constants::{SERIES_MAX_DOWNS, SERIES_TARGET_ADVANCE_MIRIM};
use crate::attributes::PlayerAttributeTable;
use crate::open_play::CarrierDecisionEvaluator;
use crate::physical::PhysicalState;
use crate::spatial::proximity::calculate_distance_mirim;
use crate::world_state::GameStatePressure;
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player, Position, SlotRole};
use arlo_math::units::Position as VectorPosition;
use arlo_tactics::{DecisionEmphasis, PassingRange, PlayerInstructions};
use smallvec::SmallVec;
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_decision_utilities_from_table(
    artrine: &Player,
    artrine_table: &PlayerAttributeTable,
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
) -> SmallVec<[(ArtrineDecisionKind, f64); 5]> {
    let down = if is_last_down {
        SERIES_MAX_DOWNS
    } else {
        SERIES_MAX_DOWNS.saturating_sub(remaining_downs).max(1)
    };
    let remaining_advance_mirim = (SERIES_TARGET_ADVANCE_MIRIM - territory_advance_mirim).max(0.0);
    let distance_to_next_artro_mirim = calculate_distance_mirim(artrine_pos, next_artro_pos);
    let epv_model = DynamicEpvModel::new(offensive_gravity);
    let current_epv = epv_model.calculate_epa(
        normalized_proximity,
        down,
        remaining_advance_mirim,
        drives_in_current_series,
    );

    let ctx = DecisionEvaluationContext::new(
        artrine,
        artrine_table,
        Position::Artrine,
        SlotRole::Standard,
        PlayerInstructions::default(),
        *artrine_physical_state,
        attribute_keys,
        epv_model,
        current_epv,
        normalized_proximity,
        drives_in_current_series,
        down,
        remaining_advance_mirim,
        pass_protection_net_advantage,
        best_available_target_weight,
        long_launch_target_weight,
        pitch_control_ahead,
        distance_to_next_artro_mirim,
        pitch_length_mirim,
        85.0,
        artrine_pos,
        offensive_gravity,
        passing_range,
        risk_profile,
        game_state_pressure,
        play_call_emphasis,
        true,
        expected_free_path_mirim,
    );

    CarrierDecisionEvaluator::evaluate_action_utilities(&ctx, available_kinds)
}

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
    risk_profile: RiskProfile,
    game_state_pressure: GameStatePressure,
    play_call_emphasis: DecisionEmphasis,
    artrine_physical_state: &PhysicalState,
    expected_free_path_mirim: f64,
) -> SmallVec<[(ArtrineDecisionKind, f64); 5]> {
    let table = PlayerAttributeTable::from_player(artrine, attribute_keys);
    calculate_decision_utilities_from_table(
        artrine,
        &table,
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
        expected_free_path_mirim,
    )
}