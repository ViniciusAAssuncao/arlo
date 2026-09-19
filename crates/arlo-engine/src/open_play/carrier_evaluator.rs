use crate::ai::evaluators::action_configs::get_action_config;
use crate::ai::evaluators::context::DecisionEvaluationContext;
use crate::ai::evaluators::generic_evaluator::evaluate_action_utility;
use arlo_domain::ArtrineDecisionKind;
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CarrierDecisionEvaluator;

impl CarrierDecisionEvaluator {
    pub fn is_rule_eligible(
        ctx: &DecisionEvaluationContext<'_>,
        kind: ArtrineDecisionKind,
    ) -> bool {
        let config = get_action_config(kind);
        (config.rule_validator_fn)(ctx)
    }

    pub fn evaluate_action_utilities(
        ctx: &DecisionEvaluationContext<'_>,
        available_kinds: &[ArtrineDecisionKind],
    ) -> SmallVec<[(ArtrineDecisionKind, f64); 5]> {
        let mut results = SmallVec::with_capacity(available_kinds.len());
        for &kind in available_kinds {
            let config = get_action_config(kind);
            if !(config.rule_validator_fn)(ctx) {
                results.push((kind, 0.0));
                continue;
            }
            let utility = evaluate_action_utility(ctx, &config);
            results.push((kind, utility));
        }
        results
    }
}