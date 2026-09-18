use crate::ai::evaluators::action_configs::types::{
    ActionEvaluationConfig, ActionKindConfig, ProgressionConfig,
};
use arlo_domain::ArtrineDecisionKind;

pub fn long_launch_config() -> ActionEvaluationConfig {
    ActionEvaluationConfig {
        decision_kind: ArtrineDecisionKind::LongLaunch,
        profile_fn: crate::artrine::decision_profiles::long_launch_profile,
        rating_additive_weight: 0.20,
        kind_config: ActionKindConfig::Progression(ProgressionConfig {
            advance_fn: |ctx, skill_mult| {
                let free_path = ctx.expected_free_path() * 1.1;
                let adv_mirim = (free_path
                    * crate::team_identity::long_launch_advance_multiplier(ctx.passing_range)
                    + ctx.long_launch_target_quality().max(0.0) * 1.8)
                    * skill_mult;
                (adv_mirim, 0)
            },
            success_prob_fn: |ctx, skill_mult| {
                0.30 + 0.03 * ctx.pass_protection_net_advantage
                    + 0.22 * ctx.long_launch_target_quality()
                    + 0.10 * skill_mult
            },
            turnover_scale: 0.38,
            urgency_bonus_fn: |_, _, _| 0.0,
        }),
        rule_validator_fn: |_ctx| true,
    }
}