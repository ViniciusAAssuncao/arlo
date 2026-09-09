use crate::ai::cognitive::RiskProfile;
use crate::ai::epv::DynamicEpvModel;
use crate::ai::evaluators::{
    ActionUtilityEvaluator, CarryUtilityEvaluator, CrossUtilityEvaluator,
    DecisionEvaluationContext, LongLaunchUtilityEvaluator, SelfFinishUtilityEvaluator,
    ShortPassUtilityEvaluator,
};
use crate::physical::PhysicalState;
use crate::world_state::GameStatePressure;
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player};
use arlo_tactics::DecisionEmphasis;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MarkovDecisionEvaluator;

impl MarkovDecisionEvaluator {
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
        risk_profile: RiskProfile,
        game_state_pressure: GameStatePressure,
        play_call_emphasis: DecisionEmphasis,
        artrine_physical_state: &PhysicalState,
    ) -> Vec<(ArtrineDecisionKind, f64)> {
        let epv_model = DynamicEpvModel::new(offensive_gravity);
        let current_epv = epv_model.calculate_epa(
            normalized_proximity,
            down,
            remaining_advance_mirim,
            drives_in_series,
        );

        let ctx = DecisionEvaluationContext {
            artrine,
            artrine_physical_state: *artrine_physical_state,
            attribute_keys,
            epv_model,
            current_epv,
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
            risk_profile,
            game_state_pressure,
            play_call_emphasis,
        };

        let carry_evaluator = CarryUtilityEvaluator;
        let short_pass_evaluator = ShortPassUtilityEvaluator;
        let long_launch_evaluator = LongLaunchUtilityEvaluator;
        let cross_evaluator = CrossUtilityEvaluator;
        let finish_evaluator = SelfFinishUtilityEvaluator;

        let mut results = Vec::with_capacity(available_kinds.len());

        for &kind in available_kinds {
            let utility = match kind {
                ArtrineDecisionKind::SelfCarry => carry_evaluator.evaluate(&ctx),
                ArtrineDecisionKind::ShortPass => short_pass_evaluator.evaluate(&ctx),
                ArtrineDecisionKind::LongLaunch => long_launch_evaluator.evaluate(&ctx),
                ArtrineDecisionKind::Cross => cross_evaluator.evaluate(&ctx),
                ArtrineDecisionKind::SelfFinish => finish_evaluator.evaluate(&ctx),
            };
            results.push((kind, utility));
        }

        results
    }
}