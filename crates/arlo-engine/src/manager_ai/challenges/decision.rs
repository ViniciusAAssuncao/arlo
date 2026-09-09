use crate::ai::cognitive::decision_threshold::action_probability;
use crate::manager_ai::context::ManagerDecisionContext;
use crate::officiating::ReviewableCall;
use arlo_domain::sport_constants::manager_cognition::{
    DECISION_THRESHOLD_LOGIT_STEEPNESS, SIGNAL_DETECTION_BASE_SENSITIVITY,
    SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
};
use arlo_domain::sport_constants::CHALLENGE_CALLS_PER_MATCH;

pub struct ChallengeDecisionEngine;

impl ChallengeDecisionEngine {
    pub fn evaluate(
        context: &ManagerDecisionContext,
        _call: &ReviewableCall,
        perceived_bad: bool,
    ) -> bool {
        if context.remaining_challenges == 0 || !perceived_bad {
            return false;
        }

        let total_challenges = CHALLENGE_CALLS_PER_MATCH.max(1) as f64;
        let challenge_ratio = (context.remaining_challenges as f64) / total_challenges;
        let stimulus = challenge_ratio.clamp(0.0, 1.0);

        let prob = action_probability(
            stimulus,
            context.manager_snapshot.challenge_judgment,
            SIGNAL_DETECTION_BASE_SENSITIVITY,
            SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
            DECISION_THRESHOLD_LOGIT_STEEPNESS,
        );

        prob.value() >= 0.5
    }
}