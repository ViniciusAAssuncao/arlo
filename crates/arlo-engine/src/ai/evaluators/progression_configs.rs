use crate::ai::evaluators::action_evalutors::{ActionEvaluationConfig, ActionKindConfig};
use crate::ai::evaluators::context::DecisionEvaluationContext;
use arlo_domain::ArtrineDecisionKind;

#[derive(Clone, Copy)]
pub struct ProgressionConfig {
    pub advance_fn: fn(&DecisionEvaluationContext<'_>, f64) -> (f64, u32),
    pub success_prob_fn: fn(&DecisionEvaluationContext<'_>, f64) -> f64,
    pub turnover_scale: f64,
    pub urgency_bonus_fn: fn(&DecisionEvaluationContext<'_>, u32, f64) -> f64,
}

pub fn carry_config() -> ActionEvaluationConfig {
    ActionEvaluationConfig {
        decision_kind: ArtrineDecisionKind::SelfCarry,
        profile_fn: crate::caching::get_cached_decision_profile,
        rating_additive_weight: 0.20,
        kind_config: ActionKindConfig::Progression(ProgressionConfig {
            advance_fn: |ctx, skill_mult| {
                let free_path = ctx.situation.expected_free_path;
                let adv_mirim = free_path * skill_mult;
                let estimated_drives = if ctx.situation.is_true_artrine && adv_mirim >= 2.0 {
                    if adv_mirim >= 8.5 && skill_mult >= 1.4 {
                        3
                    } else if adv_mirim >= 4.5 && skill_mult >= 1.0 {
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
                0.40 + 0.35 * ctx.situation.pitch_control
                    + 0.15 * skill_mult
                    + 0.03 * ctx.situation.pass_protection_net_advantage
            },
            turnover_scale: 0.12,
            urgency_bonus_fn: |ctx, estimated_drives, skill_mult| {
                if ctx.situation.is_true_artrine && ctx.situation.drives_in_series < 3 && estimated_drives > 0 {
                    (estimated_drives as f64)
                        * ((3 - ctx.situation.drives_in_series) as f64)
                        * 0.45
                        * skill_mult
                } else {
                    0.0
                }
            },
        }),
    }
}

pub fn short_pass_config() -> ActionEvaluationConfig {
    ActionEvaluationConfig {
        decision_kind: ArtrineDecisionKind::ShortPass,
        profile_fn: crate::caching::get_cached_decision_profile,
        rating_additive_weight: 0.20,
        kind_config: ActionKindConfig::Progression(ProgressionConfig {
            advance_fn: |ctx, skill_mult| {
                let free_path = ctx.situation.expected_free_path;
                let adv_mirim = (free_path
                    * crate::team_identity::short_pass_advance_multiplier(ctx.situation.passing_range)
                    + ctx.situation.target_quality.max(0.0) * 1.5)
                    * skill_mult;
                (adv_mirim, 0)
            },
            success_prob_fn: |ctx, skill_mult| {
                0.50 + 0.04 * ctx.situation.pass_protection_net_advantage
                    + 0.20 * ctx.situation.target_quality
                    + 0.10 * skill_mult
            },
            turnover_scale: 0.20,
            urgency_bonus_fn: |_, _, _| 0.0,
        }),
    }
}

pub fn long_launch_config() -> ActionEvaluationConfig {
    ActionEvaluationConfig {
        decision_kind: ArtrineDecisionKind::LongLaunch,
        profile_fn: crate::caching::get_cached_decision_profile,
        rating_additive_weight: 0.20,
        kind_config: ActionKindConfig::Progression(ProgressionConfig {
            advance_fn: |ctx, skill_mult| {
                let free_path = ctx.situation.expected_free_path * 2.5;
                let adv_mirim = (free_path
                    * crate::team_identity::long_launch_advance_multiplier(ctx.situation.passing_range)
                    + ctx.situation.long_launch_target_quality.max(0.0) * 3.0)
                    * skill_mult;
                (adv_mirim, 0)
            },
            success_prob_fn: |ctx, skill_mult| {
                0.35 + 0.03 * ctx.situation.pass_protection_net_advantage
                    + 0.25 * ctx.situation.long_launch_target_quality
                    + 0.10 * skill_mult
            },
            turnover_scale: 0.35,
            urgency_bonus_fn: |_, _, _| 0.0,
        }),
    }
}
