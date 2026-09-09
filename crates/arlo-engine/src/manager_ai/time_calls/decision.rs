use crate::ai::cognitive::decision_threshold::action_probability;
use crate::manager_ai::cognition::derive_manager_decision_noise;
use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::time_calls::urgency::compute_urgency;
use arlo_domain::sport_constants::manager_cognition::{
    DECISION_THRESHOLD_LOGIT_STEEPNESS, SIGNAL_DETECTION_BASE_SENSITIVITY,
    SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
};
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
        let noise = derive_manager_decision_noise(context.manager_snapshot.discipline).sample(rng);
        let stimulus = (urgency + noise).clamp(0.0, 1.0);

        let prob = action_probability(
            stimulus,
            context.manager_snapshot.time_call_management,
            SIGNAL_DETECTION_BASE_SENSITIVITY,
            SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
            DECISION_THRESHOLD_LOGIT_STEEPNESS,
        );
        prob.sample(rng)
    }
}
