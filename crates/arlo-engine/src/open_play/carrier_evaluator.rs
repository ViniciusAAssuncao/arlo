use crate::ai::evaluators::{
    ActionUtilityEvaluator, CarryUtilityEvaluator, CrossUtilityEvaluator, DecisionEvaluationContext,
    LongLaunchUtilityEvaluator, SelfFinishUtilityEvaluator, ShortPassUtilityEvaluator,
};
use arlo_domain::ArtrineDecisionKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CarrierDecisionEvaluator;

impl CarrierDecisionEvaluator {
    pub fn evaluate_action_utilities(
        ctx: &DecisionEvaluationContext<'_>,
        available_kinds: &[ArtrineDecisionKind],
    ) -> Vec<(ArtrineDecisionKind, f64)> {
        let carry_evaluator = CarryUtilityEvaluator;
        let short_pass_evaluator = ShortPassUtilityEvaluator;
        let long_launch_evaluator = LongLaunchUtilityEvaluator;
        let cross_evaluator = CrossUtilityEvaluator;
        let finish_evaluator = SelfFinishUtilityEvaluator;

        let mut results = Vec::with_capacity(available_kinds.len());

        for &kind in available_kinds {
            let utility = match kind {
                ArtrineDecisionKind::SelfCarry => carry_evaluator.evaluate(ctx),
                ArtrineDecisionKind::ShortPass => short_pass_evaluator.evaluate(ctx),
                ArtrineDecisionKind::LongLaunch => long_launch_evaluator.evaluate(ctx),
                ArtrineDecisionKind::Cross => cross_evaluator.evaluate(ctx),
                ArtrineDecisionKind::SelfFinish => finish_evaluator.evaluate(ctx),
            };
            results.push((kind, utility));
        }

        results
    }
}