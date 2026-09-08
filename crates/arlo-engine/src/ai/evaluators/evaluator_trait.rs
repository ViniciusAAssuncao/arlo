use crate::ai::evaluators::context::DecisionEvaluationContext;
use arlo_domain::ArtrineDecisionKind;

pub trait ActionUtilityEvaluator {
    fn decision_kind(&self) -> ArtrineDecisionKind;
    fn evaluate(&self, ctx: &DecisionEvaluationContext) -> f64;
}
