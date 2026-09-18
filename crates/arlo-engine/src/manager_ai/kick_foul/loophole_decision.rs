use crate::ai::cognitive::ManagerDecisionFactory;
use crate::ai::epv::DynamicEpvModel;
use crate::manager_ai::context::ManagerDecisionContext;
use arlo_domain::sport_constants::{
    FIELD_POINT_REQUIRED_DRIVES, FIELD_POINT_VALUE, GOAL_POINT_REQUIRED_DRIVES, GOAL_POINT_VALUE,
    KICK_FOUL_REALIGNMENT_VALUE_STEEPNESS, MINIMUM_ADVANCE_MIRINS_PER_SERIES,
};
use arlo_domain::KickFoulScoringTier;
use arlo_math::stats::contrast::logistic;
use rand::Rng;

pub fn calculate_kick_foul_realignment_stimulus(
    normalized_x: f64,
    tier: KickFoulScoringTier,
) -> f64 {
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

    logistic((realignment_epv - kick_epv) * KICK_FOUL_REALIGNMENT_VALUE_STEEPNESS)
}

pub fn evaluate_kick_foul_realignment<R: Rng + ?Sized>(
    context: &ManagerDecisionContext,
    normalized_x: f64,
    tier: KickFoulScoringTier,
    rng: &mut R,
) -> bool {
    if context.remaining_time_calls == 0 || context.is_bonus_phase {
        return false;
    }

    let stimulus = calculate_kick_foul_realignment_stimulus(normalized_x, tier);

    ManagerDecisionFactory::decide(
        stimulus,
        context.manager_snapshot.in_game_adjustments,
        context.manager_snapshot.discipline,
        rng,
    )
}