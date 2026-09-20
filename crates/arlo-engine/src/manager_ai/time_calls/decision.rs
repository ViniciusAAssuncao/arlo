use crate::ai::cognitive::evaluate_decision_gate;
use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::time_calls::urgency::compute_urgency;
use rand::Rng;

pub fn calculate_time_call_stimulus(context: &ManagerDecisionContext, just_conceded: bool) -> f64 {
    compute_urgency(context, just_conceded).clamp(0.0, 1.0)
}

pub struct TimeCallDecisionEngine;

impl TimeCallDecisionEngine {
    pub fn evaluate<R: Rng + ?Sized>(
        context: &ManagerDecisionContext,
        just_conceded: bool,
        rng: &mut R,
    ) -> bool {
        if context.remaining_time_calls == 0 {
            return false;
        }

        let stimulus = calculate_time_call_stimulus(context, just_conceded);
        evaluate_decision_gate(
            stimulus,
            context.manager_snapshot.time_call_management,
            context.manager_snapshot.discipline,
        )
        .sample(rng)
    }
}