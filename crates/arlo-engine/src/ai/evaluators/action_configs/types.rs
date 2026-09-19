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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CarryUtilityEvaluator;

impl ActionUtilityEvaluator for CarryUtilityEvaluator {
    fn decision_kind(&self) -> ArtrineDecisionKind {
        ArtrineDecisionKind::SelfCarry
    }
    fn evaluate(&self, ctx: &DecisionEvaluationContext<'_>) -> f64 {
        evaluate_action_utility(ctx, &super::carry_config())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShortPassUtilityEvaluator;

impl ActionUtilityEvaluator for ShortPassUtilityEvaluator {
    fn decision_kind(&self) -> ArtrineDecisionKind {
        ArtrineDecisionKind::ShortPass
    }
    fn evaluate(&self, ctx: &DecisionEvaluationContext<'_>) -> f64 {
        evaluate_action_utility(ctx, &super::short_pass_config())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LongLaunchUtilityEvaluator;

impl ActionUtilityEvaluator for LongLaunchUtilityEvaluator {
    fn decision_kind(&self) -> ArtrineDecisionKind {
        ArtrineDecisionKind::LongLaunch
    }
    fn evaluate(&self, ctx: &DecisionEvaluationContext<'_>) -> f64 {
        evaluate_action_utility(ctx, &super::long_launch_config())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CrossUtilityEvaluator;

impl ActionUtilityEvaluator for CrossUtilityEvaluator {
    fn decision_kind(&self) -> ArtrineDecisionKind {
        ArtrineDecisionKind::Cross
    }
    fn evaluate(&self, ctx: &DecisionEvaluationContext<'_>) -> f64 {
        evaluate_action_utility(ctx, &super::cross_config())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelfFinishUtilityEvaluator;

impl ActionUtilityEvaluator for SelfFinishUtilityEvaluator {
    fn decision_kind(&self) -> ArtrineDecisionKind {
        ArtrineDecisionKind::SelfFinish
    }
    fn evaluate(&self, ctx: &DecisionEvaluationContext<'_>) -> f64 {
        evaluate_action_utility(ctx, &super::finish_config())
    }
}