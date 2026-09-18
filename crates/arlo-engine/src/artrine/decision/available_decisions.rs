use crate::artrine::constants::OPPORTUNITY_EVALUATION_DEFAULT_RATING;
use crate::match_decision::scoring::{evaluate_scoring_opportunity, ScoringOpportunity};
use arlo_domain::ArtrineDecisionKind;
use smallvec::{smallvec, SmallVec};

pub fn available_decision_kinds(
    drives_in_current_series: u32,
    accumulated_advance_mirim: f64,
    _is_last_down: bool,
    is_bonus_phase: bool,
) -> SmallVec<[ArtrineDecisionKind; 5]> {
    let mut kinds = smallvec![
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

    if opportunity != ScoringOpportunity::None {
        kinds.push(ArtrineDecisionKind::Cross);
        kinds.push(ArtrineDecisionKind::SelfFinish);
    }

    kinds
}