use crate::ai::cognitive::evaluate_decision_gate;
use crate::manager_ai::challenges::decision::calculate_challenge_stimulus;
use crate::manager_ai::context::ManagerDecisionContext;
use crate::world_state::match_state::foul_review::FoulReviewRecord;
use arlo_domain::sport_constants::FOUL_CHALLENGE_PUNISHMENT_SEVERITY_WEIGHT;
use rand::Rng;

pub fn calculate_foul_challenge_stimulus(
    context: &ManagerDecisionContext,
    record: &FoulReviewRecord,
) -> f64 {
    let base_stimulus = calculate_challenge_stimulus(context);
    let severity_bonus = record.punishment.kind.relative_severity()
        * FOUL_CHALLENGE_PUNISHMENT_SEVERITY_WEIGHT;
    (base_stimulus + severity_bonus).clamp(0.0, 1.0)
}

pub fn evaluate_foul_challenge<R: Rng + ?Sized>(
    context: &ManagerDecisionContext,
    record: &FoulReviewRecord,
    perceived_bad: bool,
    rng: &mut R,
) -> bool {
    if context.remaining_challenges == 0 || !perceived_bad {
        return false;
    }

    let stimulus = calculate_foul_challenge_stimulus(context, record);
    evaluate_decision_gate(
        stimulus,
        context.manager_snapshot.challenge_judgment,
        context.manager_snapshot.discipline,
    )
    .sample(rng)
}