use crate::ai::evaluators::DecisionEvaluationContext;
use crate::open_play::CarrierDecisionEvaluator;
use arlo_domain::ArtrineDecisionKind;
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MarkovDecisionEvaluator;

impl MarkovDecisionEvaluator {
    pub fn evaluate_action_utilities(
        ctx: &DecisionEvaluationContext<'_>,
        available_kinds: &[ArtrineDecisionKind],
    ) -> SmallVec<[(ArtrineDecisionKind, f64); 5]> {
        CarrierDecisionEvaluator::evaluate_action_utilities(ctx, available_kinds)
    }
}