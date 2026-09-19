use crate::artrine::constants::FIELD_POINT_OPPORTUNITY_ADVANCE_BUFFER_MIRIM;
use crate::scoring_regime::{can_attempt_cross_or_finish, ScoringRegimePolicy};
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

    let regime = ScoringRegimePolicy::default();

    if can_attempt_cross_or_finish(
        &regime,
        is_bonus_phase,
        drives_in_current_series,
        accumulated_advance_mirim,
        FIELD_POINT_OPPORTUNITY_ADVANCE_BUFFER_MIRIM,
    ) {
        kinds.push(ArtrineDecisionKind::Cross);
        kinds.push(ArtrineDecisionKind::SelfFinish);
    }

    kinds
}