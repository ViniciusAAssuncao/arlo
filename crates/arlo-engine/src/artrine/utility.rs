use crate::ai::markov_decision::MarkovDecisionEvaluator;
use crate::match_decision::scoring::{evaluate_scoring_opportunity, ScoringOpportunity};
use crate::spatial::proximity::calculate_distance_mirim;
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player};
use arlo_math::units::Position as VectorPosition;
use std::collections::HashMap;
use uuid::Uuid;

pub fn available_decision_kinds(
    drives_in_current_series: u32,
    accumulated_advance_mirim: f64,
    _is_last_down: bool,
    is_bonus_phase: bool,
) -> Vec<ArtrineDecisionKind> {
    let mut kinds = vec![
        ArtrineDecisionKind::SelfCarry,
        ArtrineDecisionKind::ShortPass,
        ArtrineDecisionKind::LongLaunch,
    ];

    let opportunity = evaluate_scoring_opportunity(
        is_bonus_phase,
        drives_in_current_series,
        accumulated_advance_mirim,
        10.0,
    );

    if opportunity != ScoringOpportunity::None {
        kinds.push(ArtrineDecisionKind::Cross);
        kinds.push(ArtrineDecisionKind::SelfFinish);
    }

    kinds
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
    artrine_pos: VectorPosition,
    next_artro_pos: VectorPosition,
    pitch_control_ahead: f64,
    pitch_length_mirim: f64,
) -> Vec<(ArtrineDecisionKind, f64)> {
    let down = (4u8).saturating_sub(remaining_downs).max(1);
    let remaining_advance_mirim = (10.0 - territory_advance_mirim).max(0.0);
    let distance_to_next_artro_mirim = calculate_distance_mirim(artrine_pos, next_artro_pos);

    MarkovDecisionEvaluator::evaluate_action_utilities(
        artrine,
        attribute_keys,
        available_kinds,
        normalized_proximity,
        drives_in_current_series,
        if is_last_down { 4 } else { down },
        remaining_advance_mirim,
        pass_protection_net_advantage,
        best_available_target_weight,
        pitch_control_ahead,
        distance_to_next_artro_mirim,
        pitch_length_mirim,
    )
}
