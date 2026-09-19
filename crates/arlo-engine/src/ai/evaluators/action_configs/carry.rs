use crate::ai::evaluators::action_configs::types::{
    ActionEvaluationConfig, ActionKindConfig, ProgressionConfig,
};
use arlo_domain::ArtrineDecisionKind;

pub fn carry_config() -> ActionEvaluationConfig {
    ActionEvaluationConfig {
        decision_kind: ArtrineDecisionKind::SelfCarry,
        profile_fn: crate::artrine::decision_profiles::self_carry_profile,
        rating_additive_weight: 0.20,
        kind_config: ActionKindConfig::Progression(ProgressionConfig {
            advance_fn: |ctx, skill_mult| {
                let free_path = ctx.expected_free_path();
                let adv_mirim = free_path * skill_mult * 0.45;
                let estimated_drives = if ctx.is_true_artrine && adv_mirim >= 1.5 {
                    if adv_mirim >= 6.5 && skill_mult >= 1.4 {
                        3
                    } else if adv_mirim >= 3.5 && skill_mult >= 1.0 {
                        2
                    } else {
                        1
                    }
                } else {
                    0
                };
                (adv_mirim, estimated_drives)
            },
            success_prob_fn: |ctx, skill_mult| {
                0.35 + 0.30 * ctx.pitch_control()
                    + 0.12 * skill_mult
                    + 0.03 * ctx.pass_protection_net_advantage
            },
            turnover_scale: 0.14,
            urgency_bonus_fn: |ctx, estimated_drives, skill_mult| {
                if ctx.is_true_artrine && ctx.drives_in_series < 3 && estimated_drives > 0 {
                    (estimated_drives as f64)
                        * ((3 - ctx.drives_in_series) as f64)
                        * 0.35
                        * skill_mult
                } else {
                    0.0
                }
            },
        }),
        rule_validator_fn: |_ctx| true,
    }
}