use crate::ai::evaluators::action_configs::types::{
    ActionEvaluationConfig, ActionKindConfig, TerminalScoreConfig,
};
use arlo_domain::sport_constants::{
    FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM, FIELD_POINT_REQUIRED_DRIVES,
    GOAL_POINT_REQUIRED_DRIVES,
};
use arlo_domain::ArtrineDecisionKind;

pub fn cross_config() -> ActionEvaluationConfig {
    ActionEvaluationConfig {
        decision_kind: ArtrineDecisionKind::Cross,
        profile_fn: crate::artrine::decision_profiles::cross_profile,
        rating_additive_weight: 0.20,
        kind_config: ActionKindConfig::TerminalScore(TerminalScoreConfig {
            success_prob_fn: |ctx, skill_mult| {
                (0.35 + 0.35 * ctx.normalized_proximity
                    + 0.20 * ctx.target_quality()
                    + 0.10 * skill_mult)
                    * (0.60 + 0.40 * ctx.offensive_gravity.min(2.0))
                    * (0.80 + 0.25 * ctx.lateral_ratio())
            },
            geometry_factor_fn: |ctx| 0.70 + 0.60 * ctx.lateral_ratio(),
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