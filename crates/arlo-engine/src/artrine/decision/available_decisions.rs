use crate::artrine::constants::FIELD_POINT_OPPORTUNITY_ADVANCE_BUFFER_MIRIM;
use crate::scoring_regime::{can_attempt_cross_or_finish, ScoringRegimePolicy};
use arlo_domain::ArtrineDecisionKind;
use smallvec::{smallvec, SmallVec};

pub fn available_decision_kinds(
    drives_in_current_series: u32,
    possession_advance_mirim: f64,
    is_last_down: bool,
    is_bonus_phase: bool,
    down: u8,
    normalized_proximity: f64,
    _is_true_artrine: bool,
) -> SmallVec<[ArtrineDecisionKind; 5]> {
    let regime = ScoringRegimePolicy::default();

    let can_score = can_attempt_cross_or_finish(
        &regime,
        is_bonus_phase,
        drives_in_current_series,
        possession_advance_mirim,
        FIELD_POINT_OPPORTUNITY_ADVANCE_BUFFER_MIRIM,
        down,
        normalized_proximity,
    );

    if (is_last_down || is_bonus_phase) && can_score {
        return smallvec![
            ArtrineDecisionKind::Cross,
            ArtrineDecisionKind::SelfFinish,
        ];
    }

    let mut kinds = smallvec![
        ArtrineDecisionKind::SelfCarry,
        ArtrineDecisionKind::ShortPass,
        ArtrineDecisionKind::LongLaunch,
    ];

    if can_score {
        kinds.push(ArtrineDecisionKind::Cross);
        kinds.push(ArtrineDecisionKind::SelfFinish);
    }

    kinds
}
