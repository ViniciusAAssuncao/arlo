use crate::ai::evaluators::DecisionEvaluationContext;
use crate::open_play::CarrierDecisionEvaluator;
use arlo_domain::ArtrineDecisionKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MarkovDecisionEvaluator;

impl MarkovDecisionEvaluator {
    pub fn evaluate_action_utilities(
        ctx: &DecisionEvaluationContext<'_>,
        available_kinds: &[ArtrineDecisionKind],
    ) -> Vec<(ArtrineDecisionKind, f64)> {
        CarrierDecisionEvaluator::evaluate_action_utilities(ctx, available_kinds)
    }
}