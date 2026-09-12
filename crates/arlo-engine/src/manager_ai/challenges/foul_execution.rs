use crate::manager_ai::challenges::foul_decision::evaluate_foul_challenge;
use crate::manager_ai::challenges::foul_perception::perceives_bad_foul_call;
use crate::manager_ai::cognition::ManagerDecisionKind;
use crate::manager_ai::context::ManagerDecisionContext;
use crate::officiating::punishment::reverse_punishment;
use crate::officiating::ReviewableCallKind;
use crate::world_state::match_state::foul_review::FoulReviewRecord;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;
use rand::Rng;
use uuid::Uuid;

pub fn execute_foul_challenge<R: Rng + ?Sized>(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    context: &ManagerDecisionContext,
    team_id: Uuid,
    record: &FoulReviewRecord,
    rng: &mut R,
) -> bool {
    let perceived_bad = perceives_bad_foul_call(
        record.original_call_correct,
        context.manager_snapshot.challenge_judgment,
        context.manager_snapshot.judging_ability,
        rng,
    );

    if !evaluate_foul_challenge(context, record, perceived_bad, rng) {
        return false;
    }

    let success = !record.original_call_correct;
    let is_home = context.is_home;
    let used = publisher
        .state_mut()
        .clock_mut()
        .use_challenge(is_home, success);
    if !used {
        return false;
    }

    publisher
        .state_mut()
        .mark_decision_triggered(team_id, ManagerDecisionKind::Challenge);

    if success {
        reverse_punishment(publisher.state_mut(), &record.punishment);
    }

    let remaining_challenges_after = if is_home {
        publisher.state().clock().home_challenges()
    } else {
        publisher.state().clock().away_challenges()
    };

    publisher.emit_challenge_resolved(
        team_id,
        ReviewableCallKind::FoulClassification,
        success,
        remaining_challenges_after,
    );

    publisher.state_mut().clear_last_reviewable_foul();

    success
}