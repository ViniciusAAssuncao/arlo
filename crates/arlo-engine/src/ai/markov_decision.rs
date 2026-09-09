use crate::ai::cognitive::RiskProfile;
use crate::open_play::CarrierDecisionEvaluator;
use crate::physical::PhysicalState;
use crate::world_state::GameStatePressure;
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player, Position, SlotRole};
use arlo_math::units::Position as VectorPosition;
use arlo_tactics::{DecisionEmphasis, PassingRange, PlayerInstructions};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MarkovDecisionEvaluator;

impl MarkovDecisionEvaluator {
    pub fn evaluate_carrier_action_utilities(
        carrier: &Player,
        carrier_position: Position,
        carrier_role: SlotRole,
        carrier_instructions: PlayerInstructions,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
        available_kinds: &[ArtrineDecisionKind],
        normalized_proximity: f64,
        drives_in_series: u32,
        down: u8,
        remaining_advance_mirim: f64,
        pass_protection_net_advantage: f64,
        best_available_target_weight: f64,
        long_launch_target_weight: f64,
        carrier_pos_vec: VectorPosition,
        pitch_control_ahead: f64,
        distance_to_next_artro_mirim: f64,
        pitch_length_mirim: f64,
        pitch_width_mirim: f64,
        offensive_gravity: f64,
        passing_range: PassingRange,
        risk_profile: RiskProfile,
        game_state_pressure: GameStatePressure,
        play_call_emphasis: DecisionEmphasis,
        carrier_physical_state: &PhysicalState,
        is_true_artrine: bool,
        expected_free_path_mirim: f64,
    ) -> Vec<(ArtrineDecisionKind, f64)> {
        CarrierDecisionEvaluator::evaluate_action_utilities(
            carrier,
            carrier_position,
            carrier_role,
            carrier_instructions,
            attribute_keys,
            available_kinds,
            normalized_proximity,
            drives_in_series,
            down,
            remaining_advance_mirim,
            pass_protection_net_advantage,
            best_available_target_weight,
            long_launch_target_weight,
            carrier_pos_vec,
            pitch_control_ahead,
            distance_to_next_artro_mirim,
            pitch_length_mirim,
            pitch_width_mirim,
            offensive_gravity,
            passing_range,
            risk_profile,
            game_state_pressure,
            play_call_emphasis,
            carrier_physical_state,
            is_true_artrine,
            expected_free_path_mirim,
        )
    }

    pub fn evaluate_action_utilities(
        artrine: &Player,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
        available_kinds: &[ArtrineDecisionKind],
        normalized_proximity: f64,
        drives_in_series: u32,
        down: u8,
        remaining_advance_mirim: f64,
        pass_protection_net_advantage: f64,
        best_available_target_weight: f64,
        long_launch_target_weight: f64,
        pitch_control_ahead: f64,
        distance_to_next_artro_mirim: f64,
        pitch_length_mirim: f64,
        offensive_gravity: f64,
        passing_range: PassingRange,
        play_call_emphasis: DecisionEmphasis,
        artrine_physical_state: &PhysicalState,
    ) -> Vec<(ArtrineDecisionKind, f64)> {
        let risk_profile =
            RiskProfile::from_player(artrine, attribute_keys, artrine_physical_state);
        let game_state_pressure = GameStatePressure::default();

        Self::evaluate_action_utilities_with_context(
            artrine,
            attribute_keys,
            available_kinds,
            normalized_proximity,
            drives_in_series,
            down,
            remaining_advance_mirim,
            pass_protection_net_advantage,
            best_available_target_weight,
            long_launch_target_weight,
            pitch_control_ahead,
            distance_to_next_artro_mirim,
            pitch_length_mirim,
            offensive_gravity,
            passing_range,
            risk_profile,
            game_state_pressure,
            play_call_emphasis,
            artrine_physical_state,
        )
    }

    pub fn evaluate_action_utilities_with_context(
        artrine: &Player,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
        available_kinds: &[ArtrineDecisionKind],
        normalized_proximity: f64,
        drives_in_series: u32,
        down: u8,
        remaining_advance_mirim: f64,
        pass_protection_net_advantage: f64,
        best_available_target_weight: f64,
        long_launch_target_weight: f64,
        pitch_control_ahead: f64,
        distance_to_next_artro_mirim: f64,
        pitch_length_mirim: f64,
        offensive_gravity: f64,
        passing_range: PassingRange,
        risk_profile: RiskProfile,
        game_state_pressure: GameStatePressure,
        play_call_emphasis: DecisionEmphasis,
        artrine_physical_state: &PhysicalState,
    ) -> Vec<(ArtrineDecisionKind, f64)> {
        Self::evaluate_carrier_action_utilities(
            artrine,
            Position::Artrine,
            SlotRole::Standard,
            PlayerInstructions::default(),
            attribute_keys,
            available_kinds,
            normalized_proximity,
            drives_in_series,
            down,
            remaining_advance_mirim,
            pass_protection_net_advantage,
            best_available_target_weight,
            long_launch_target_weight,
            VectorPosition::zero(),
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
            0.0,
        )
    }
}