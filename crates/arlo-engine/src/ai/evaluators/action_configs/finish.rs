use crate::ai::evaluators::action_configs::types::{
    ActionEvaluationConfig, ActionKindConfig, TerminalScoreConfig,
};
use arlo_domain::sport_constants::{
    FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM, FIELD_POINT_REQUIRED_DRIVES,
    GOAL_POINT_REQUIRED_DRIVES,
};
use arlo_domain::ArtrineDecisionKind;

pub fn finish_config() -> ActionEvaluationConfig {
    ActionEvaluationConfig {
        decision_kind: ArtrineDecisionKind::SelfFinish,
        profile_fn: crate::artrine::decision_profiles::self_finish_profile,
        rating_additive_weight: 0.25,
        kind_config: ActionKindConfig::TerminalScore(TerminalScoreConfig {
            success_prob_fn: |ctx, skill_mult| {
                let distance_to_goal_mirim =
                    ((1.0 - ctx.normalized_proximity) * ctx.pitch_length_mirim()).max(0.0);
                let distance_p_factor =
                    (1.0 / (1.0 + distance_to_goal_mirim * 0.06)).clamp(0.15, 1.0);
                let exhaustion_penalty = 1.0 - 0.45 * ctx.risk_profile.physical_exhaustion();
                let pressure_dampener = 0.55 + 0.45 * ctx.pitch_control();
                (0.15 + 0.40 * distance_p_factor + 0.20 * skill_mult + 0.15 * ctx.pitch_control())
                    * ctx.shooting_angle_factor()
                    * pressure_dampener
                    * exhaustion_penalty
            },
            geometry_factor_fn: |ctx| {
                let zone_multiplier =
                    crate::resolution::finish_distance_multiplier(ctx.normalized_proximity);
                let shooting_lane_clearance = 0.55 + 0.45 * ctx.pitch_control();
                zone_multiplier * shooting_lane_clearance
            },
        }),
        rule_validator_fn: |ctx| {
            ctx.drives_in_series >= GOAL_POINT_REQUIRED_DRIVES
                || (ctx.drives_in_series >= FIELD_POINT_REQUIRED_DRIVES
                    && ((10.0 - ctx.remaining_advance_mirim)
                        >= FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM
                        || ctx.normalized_proximity >= 0.70))
        },
    }
}