use crate::ai::cognitive::decision_threshold::action_probability;
use crate::manager_ai::challenges::decision::base_challenge_stimulus;
use crate::manager_ai::context::ManagerDecisionContext;
use crate::world_state::match_state::foul_review::FoulReviewRecord;
use arlo_domain::sport_constants::manager_cognition::{
    DECISION_THRESHOLD_LOGIT_STEEPNESS, SIGNAL_DETECTION_BASE_SENSITIVITY,
    SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
};
use arlo_domain::sport_constants::FOUL_CHALLENGE_PUNISHMENT_SEVERITY_WEIGHT;
use rand::Rng;

pub fn evaluate_foul_challenge<R: Rng + ?Sized>(
    context: &ManagerDecisionContext,
    record: &FoulReviewRecord,
    perceived_bad: bool,
    rng: &mut R,
) -> bool {
    if context.remaining_challenges == 0 || !perceived_bad {
        return false;
    }

    let base_stimulus = base_challenge_stimulus(context, rng);
    let severity_bonus = record.punishment.kind.relative_severity()
        * FOUL_CHALLENGE_PUNISHMENT_SEVERITY_WEIGHT;
    let stimulus = (base_stimulus + severity_bonus).clamp(0.0, 1.0);

    let prob = action_probability(
        stimulus,
        context.manager_snapshot.challenge_judgment,
        SIGNAL_DETECTION_BASE_SENSITIVITY,
        SIGNAL_DETECTION_JUDGMENT_ATTRIBUTE_SCALE,
        DECISION_THRESHOLD_LOGIT_STEEPNESS,
    );

    prob.sample(rng)
}