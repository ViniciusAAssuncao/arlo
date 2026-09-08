use crate::artrine::constants::{
    FIELD_POINT_OPPORTUNITY_ADVANCE_BUFFER_MIRIM, OPPORTUNITY_EVALUATION_DEFAULT_RATING,
};
use crate::match_decision::scoring::{evaluate_scoring_opportunity, ScoringOpportunity};
use arlo_domain::sport_constants::{
    FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM, FIELD_POINT_REQUIRED_DRIVES,
    GOAL_POINT_REQUIRED_DRIVES,
};
use arlo_domain::ArtrineDecisionKind;

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
        OPPORTUNITY_EVALUATION_DEFAULT_RATING,
    );

    let can_cross_or_finish = opportunity != ScoringOpportunity::None
        || is_bonus_phase
        || drives_in_current_series >= GOAL_POINT_REQUIRED_DRIVES
        || (drives_in_current_series >= FIELD_POINT_REQUIRED_DRIVES
            && accumulated_advance_mirim
                >= (FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM
                    - FIELD_POINT_OPPORTUNITY_ADVANCE_BUFFER_MIRIM));

    if can_cross_or_finish {
        kinds.push(ArtrineDecisionKind::Cross);
        kinds.push(ArtrineDecisionKind::SelfFinish);
    }

    kinds
}