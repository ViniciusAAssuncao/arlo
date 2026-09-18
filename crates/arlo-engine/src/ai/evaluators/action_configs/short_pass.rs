use crate::ai::evaluators::action_configs::types::{
    ActionEvaluationConfig, ActionKindConfig, ProgressionConfig,
};
use arlo_domain::ArtrineDecisionKind;

pub fn short_pass_config() -> ActionEvaluationConfig {
    ActionEvaluationConfig {
        decision_kind: ArtrineDecisionKind::ShortPass,
        profile_fn: crate::artrine::decision_profiles::short_pass_profile,
        rating_additive_weight: 0.20,
        kind_config: ActionKindConfig::Progression(ProgressionConfig {
            advance_fn: |ctx, skill_mult| {
                let free_path = ctx.expected_free_path() * 0.50;
                let adv_mirim = (free_path
                    * crate::team_identity::short_pass_advance_multiplier(ctx.passing_range)
                    + ctx.target_quality().max(0.0) * 0.9)
                    * skill_mult;
                (adv_mirim, 0)
            },
            success_prob_fn: |ctx, skill_mult| {
                0.45 + 0.04 * ctx.pass_protection_net_advantage
                    + 0.18 * ctx.target_quality()
                    + 0.10 * skill_mult
            },
            turnover_scale: 0.22,
            urgency_bonus_fn: |_, _, _| 0.0,
        }),
        rule_validator_fn: |_ctx| true,
    }
}