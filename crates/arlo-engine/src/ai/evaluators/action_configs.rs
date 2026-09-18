use crate::ai::evaluators::context::DecisionEvaluationContext;
use crate::ai::evaluators::evaluator_trait::ActionUtilityEvaluator;
use crate::ai::evaluators::generic_evaluator::evaluate_action_utility;
use crate::attributes::profiles::AttributeProfile;
use arlo_domain::ArtrineDecisionKind;

#[derive(Clone, Copy)]
pub struct ProgressionConfig {
    pub advance_fn: fn(&DecisionEvaluationContext<'_>, f64) -> (f64, u32),
    pub success_prob_fn: fn(&DecisionEvaluationContext<'_>, f64) -> f64,
    pub turnover_scale: f64,
    pub urgency_bonus_fn: fn(&DecisionEvaluationContext<'_>, u32, f64) -> f64,
}

#[derive(Clone, Copy)]
pub struct TerminalScoreConfig {
    pub success_prob_fn: fn(&DecisionEvaluationContext<'_>, f64) -> f64,
    pub geometry_factor_fn: fn(&DecisionEvaluationContext<'_>) -> f64,
}

#[derive(Clone, Copy)]
pub enum ActionKindConfig {
    Progression(ProgressionConfig),
    TerminalScore(TerminalScoreConfig),
}

#[derive(Clone, Copy)]
pub struct ActionEvaluationConfig {
    pub decision_kind: ArtrineDecisionKind,
    pub profile_fn: fn() -> AttributeProfile,
    pub rating_additive_weight: f64,
    pub kind_config: ActionKindConfig,
    pub rule_validator_fn: fn(&DecisionEvaluationContext<'_>) -> bool,
}

pub fn carry_config() -> ActionEvaluationConfig {
    ActionEvaluationConfig {
        decision_kind: ArtrineDecisionKind::SelfCarry,
        profile_fn: crate::artrine::decision_profiles::self_carry_profile,
        rating_additive_weight: 0.20,
        kind_config: ActionKindConfig::Progression(ProgressionConfig {
            advance_fn: |ctx, skill_mult| {
                let free_path = ctx.expected_free_path();
                let adv_mirim = free_path * skill_mult;
                let estimated_drives = if ctx.is_true_artrine && adv_mirim >= 2.0 {
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
                0.40 + 0.35 * ctx.pitch_control()
                    + 0.15 * skill_mult
                    + 0.03 * ctx.pass_protection_net_advantage
            },
            turnover_scale: 0.12,
            urgency_bonus_fn: |ctx, estimated_drives, skill_mult| {
                if ctx.is_true_artrine && ctx.drives_in_series < 3 && estimated_drives > 0 {
                    (estimated_drives as f64)
                        * ((3 - ctx.drives_in_series) as f64)
                        * 0.45
                        * skill_mult
                } else {
                    0.0
                }
            },
        }),
        rule_validator_fn: |_ctx| true,
    }
}

pub fn short_pass_config() -> ActionEvaluationConfig {
    ActionEvaluationConfig {
        decision_kind: ArtrineDecisionKind::ShortPass,
        profile_fn: crate::artrine::decision_profiles::short_pass_profile,
        rating_additive_weight: 0.20,
        kind_config: ActionKindConfig::Progression(ProgressionConfig {
            advance_fn: |ctx, skill_mult| {
                let free_path = ctx.expected_free_path();
                let adv_mirim = (free_path
                    * crate::team_identity::short_pass_advance_multiplier(ctx.passing_range)
                    + ctx.target_quality().max(0.0) * 1.5)
                    * skill_mult;
                (adv_mirim, 0)
            },
            success_prob_fn: |ctx, skill_mult| {
                0.50 + 0.04 * ctx.pass_protection_net_advantage
                    + 0.20 * ctx.target_quality()
                    + 0.10 * skill_mult
            },
            turnover_scale: 0.20,
            urgency_bonus_fn: |_, _, _| 0.0,
        }),
        rule_validator_fn: |_ctx| true,
    }
}

pub fn long_launch_config() -> ActionEvaluationConfig {
    ActionEvaluationConfig {
        decision_kind: ArtrineDecisionKind::LongLaunch,
        profile_fn: crate::artrine::decision_profiles::long_launch_profile,
        rating_additive_weight: 0.20,
        kind_config: ActionKindConfig::Progression(ProgressionConfig {
            advance_fn: |ctx, skill_mult| {
                let free_path = ctx.expected_free_path() * 2.5;
                let adv_mirim = (free_path
                    * crate::team_identity::long_launch_advance_multiplier(ctx.passing_range)
                    + ctx.long_launch_target_quality().max(0.0) * 3.0)
                    * skill_mult;
                (adv_mirim, 0)
            },
            success_prob_fn: |ctx, skill_mult| {
                0.35 + 0.03 * ctx.pass_protection_net_advantage
                    + 0.25 * ctx.long_launch_target_quality()
                    + 0.10 * skill_mult
            },
            turnover_scale: 0.35,
            urgency_bonus_fn: |_, _, _| 0.0,
        }),
        rule_validator_fn: |_ctx| true,
    }
}

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
        rule_validator_fn: |ctx| ctx.drives_in_series >= 1 || ctx.normalized_proximity >= 0.70,
    }
}

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
                    (1.0 / (1.0 + distance_to_goal_mirim * 0.05)).clamp(0.20, 1.0);
                (0.20 + 0.40 * distance_p_factor + 0.20 * skill_mult + 0.20 * ctx.pitch_control())
                    * ctx.shooting_angle_factor()
            },
            geometry_factor_fn: |ctx| {
                let zone_multiplier =
                    crate::resolution::finish_distance_multiplier(ctx.normalized_proximity);
                let shooting_lane_clearance = 0.60 + 0.40 * ctx.pitch_control();
                zone_multiplier * shooting_lane_clearance
            },
        }),
        rule_validator_fn: |ctx| ctx.drives_in_series >= 1,
    }
}

pub fn get_action_config(kind: ArtrineDecisionKind) -> ActionEvaluationConfig {
    match kind {
        ArtrineDecisionKind::SelfCarry => carry_config(),
        ArtrineDecisionKind::ShortPass => short_pass_config(),
        ArtrineDecisionKind::LongLaunch => long_launch_config(),
        ArtrineDecisionKind::Cross => cross_config(),
        ArtrineDecisionKind::SelfFinish => finish_config(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CarryUtilityEvaluator;

impl ActionUtilityEvaluator for CarryUtilityEvaluator {
    fn decision_kind(&self) -> ArtrineDecisionKind {
        ArtrineDecisionKind::SelfCarry
    }
    fn evaluate(&self, ctx: &DecisionEvaluationContext<'_>) -> f64 {
        evaluate_action_utility(ctx, &carry_config())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShortPassUtilityEvaluator;

impl ActionUtilityEvaluator for ShortPassUtilityEvaluator {
    fn decision_kind(&self) -> ArtrineDecisionKind {
        ArtrineDecisionKind::ShortPass
    }
    fn evaluate(&self, ctx: &DecisionEvaluationContext<'_>) -> f64 {
        evaluate_action_utility(ctx, &short_pass_config())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LongLaunchUtilityEvaluator;

impl ActionUtilityEvaluator for LongLaunchUtilityEvaluator {
    fn decision_kind(&self) -> ArtrineDecisionKind {
        ArtrineDecisionKind::LongLaunch
    }
    fn evaluate(&self, ctx: &DecisionEvaluationContext<'_>) -> f64 {
        evaluate_action_utility(ctx, &long_launch_config())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrossUtilityEvaluator;

impl ActionUtilityEvaluator for CrossUtilityEvaluator {
    fn decision_kind(&self) -> ArtrineDecisionKind {
        ArtrineDecisionKind::Cross
    }
    fn evaluate(&self, ctx: &DecisionEvaluationContext<'_>) -> f64 {
        evaluate_action_utility(ctx, &cross_config())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelfFinishUtilityEvaluator;

impl ActionUtilityEvaluator for SelfFinishUtilityEvaluator {
    fn decision_kind(&self) -> ArtrineDecisionKind {
        ArtrineDecisionKind::SelfFinish
    }
    fn evaluate(&self, ctx: &DecisionEvaluationContext<'_>) -> f64 {
        evaluate_action_utility(ctx, &finish_config())
    }
}