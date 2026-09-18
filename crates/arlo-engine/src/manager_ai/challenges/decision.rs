use crate::ai::cognitive::sample_manager_action;
use crate::manager_ai::cognition::sample_manager_decision_noise;
use crate::manager_ai::context::ManagerDecisionContext;
use crate::officiating::ReviewableCall;
use arlo_domain::sport_constants::{CHALLENGE_CALLS_PER_MATCH, CHALLENGE_LEVERAGE_WEIGHT};
use rand::Rng;

pub fn base_challenge_stimulus<R: Rng + ?Sized>(
    context: &ManagerDecisionContext,
    rng: &mut R,
) -> f64 {
    let total_challenges = CHALLENGE_CALLS_PER_MATCH.max(1) as f64;
    let challenge_ratio = (context.remaining_challenges as f64) / total_challenges;
    let leverage_mult =
        1.0 + context.situational_awareness.leverage() * CHALLENGE_LEVERAGE_WEIGHT;
    let noise = sample_manager_decision_noise(context.manager_snapshot.discipline, rng);
    (challenge_ratio * leverage_mult + noise).clamp(0.0, 1.0)
}

pub struct ChallengeDecisionEngine;

impl ChallengeDecisionEngine {
    pub fn evaluate<R: Rng + ?Sized>(
        context: &ManagerDecisionContext,
        _call: &ReviewableCall,
        perceived_bad: bool,
        rng: &mut R,
    ) -> bool {
        if context.remaining_challenges == 0 || !perceived_bad {
            return false;
        }

        let stimulus = base_challenge_stimulus(context, rng);
        sample_manager_action(stimulus, context.manager_snapshot.challenge_judgment, rng)
    }
}