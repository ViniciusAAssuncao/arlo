use crate::ai::cognitive::decision_threshold::action_probability;
use crate::ai::epv::DynamicEpvModel;
use crate::manager_ai::context::ManagerDecisionContext;
use arlo_domain::sport_constants::manager_cognition::{
    DECISION_THRESHOLD_LOGIT_STEEPNESS, SIGNAL_DETECTION_BASE_SENSITIVITY,
    SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
};
use arlo_domain::sport_constants::{
    FIELD_POINT_REQUIRED_DRIVES, FIELD_POINT_VALUE, GOAL_POINT_REQUIRED_DRIVES, GOAL_POINT_VALUE,
    KICK_FOUL_REALIGNMENT_VALUE_STEEPNESS, MINIMUM_ADVANCE_MIRINS_PER_SERIES,
};
use arlo_domain::KickFoulScoringTier;
use arlo_math::stats::contrast::logistic;
use rand::Rng;

pub fn evaluate_kick_foul_realignment<R: Rng + ?Sized>(
    context: &ManagerDecisionContext,
    normalized_x: f64,
    tier: KickFoulScoringTier,
    rng: &mut R,
) -> bool {
    if context.remaining_time_calls == 0 || context.is_bonus_phase {
        return false;
    }

    let epv_model = DynamicEpvModel::new(1.0);
    let realignment_epv = epv_model.calculate_epa(
        normalized_x,
        1,
        MINIMUM_ADVANCE_MIRINS_PER_SERIES,
        0,
    );

    let kick_epv = match tier {
        KickFoulScoringTier::FirstZone => {
            epv_model.goal_probability(
                normalized_x,
                GOAL_POINT_REQUIRED_DRIVES,
                1,
                MINIMUM_ADVANCE_MIRINS_PER_SERIES,
            ) * (GOAL_POINT_VALUE as f64)
        }
        KickFoulScoringTier::Standard => {
            epv_model.field_point_probability(
                normalized_x,
                FIELD_POINT_REQUIRED_DRIVES,
                1,
                MINIMUM_ADVANCE_MIRINS_PER_SERIES,
            ) * (FIELD_POINT_VALUE as f64)
        }
    };

    let stimulus =
        logistic((realignment_epv - kick_epv) * KICK_FOUL_REALIGNMENT_VALUE_STEEPNESS);

    let prob = action_probability(
        stimulus,
        context.manager_snapshot.in_game_adjustments,
        SIGNAL_DETECTION_BASE_SENSITIVITY,
        SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
        DECISION_THRESHOLD_LOGIT_STEEPNESS,
    );

    prob.sample(rng)
}