use crate::artrine::context_terms::total_context_utility;
use crate::artrine::decision_profiles::get_artrine_decision_profile;
use crate::match_decision::scoring::{evaluate_scoring_opportunity, ScoringOpportunity};
use crate::resolution::group_rating::calculate_player_duel_rating;
use arlo_domain::sport_constants::FIELD_GOAL_MIN_TERRITORY_ADVANCE_MIRIM_FIELDPOST;
use arlo_domain::{ArtrineDecisionKind, AttributeKey, Player, Position};
use std::collections::HashMap;
use uuid::Uuid;

pub fn available_decision_kinds(
    drives_in_current_series: u32,
    accumulated_advance_mirim: f64,
    is_last_down: bool,
) -> Vec<ArtrineDecisionKind> {
    let mut kinds = vec![
        ArtrineDecisionKind::SelfCarry,
        ArtrineDecisionKind::ShortPass,
        ArtrineDecisionKind::LongLaunch,
    ];

    let opportunity =
        evaluate_scoring_opportunity(drives_in_current_series, accumulated_advance_mirim);
    let field_goal_reachable = is_last_down
        && accumulated_advance_mirim >= FIELD_GOAL_MIN_TERRITORY_ADVANCE_MIRIM_FIELDPOST;

    if opportunity != ScoringOpportunity::None || field_goal_reachable {
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
) -> Vec<(ArtrineDecisionKind, f64)> {
    let mut results = Vec::with_capacity(available_kinds.len());

    for &kind in available_kinds {
        let profile = get_artrine_decision_profile(kind);
        let intrinsic_rating = calculate_player_duel_rating(
            artrine,
            Position::Artrine,
            attribute_keys,
            &profile,
        );
        let context_util = total_context_utility(
            kind,
            normalized_proximity,
            drives_in_current_series,
            remaining_downs,
            pass_protection_net_advantage,
            is_last_down,
            territory_advance_mirim,
        );
        let total_utility = intrinsic_rating + context_util;
        results.push((kind, total_utility));
    }

    results
}
