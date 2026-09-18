use crate::ai::cognitive::sample_manager_action;
use crate::manager_ai::cognition::sample_manager_decision_noise;
use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::time_calls::urgency::compute_urgency;
use rand::Rng;

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

        let urgency = compute_urgency(context, just_conceded);
        let noise = sample_manager_decision_noise(context.manager_snapshot.discipline, rng);
        let stimulus = (urgency + noise).clamp(0.0, 1.0);

        sample_manager_action(stimulus, context.manager_snapshot.time_call_management, rng)
    }
}