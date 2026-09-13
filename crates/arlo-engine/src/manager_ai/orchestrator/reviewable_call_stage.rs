use crate::manager_ai::challenges::evaluate_and_execute_challenge;
use crate::manager_ai::cognition::{derive_cooldown_seconds, ManagerDecisionKind};
use crate::manager_ai::context::ManagerDecisionContext;
use crate::manager_ai::human_control::try_apply_human_challenge;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_domain::ManagerControlMode;
use arlo_events::EventSink;
use arlo_manager_control::ManagerDecisionInbox;
use rand::Rng;
use uuid::Uuid;

pub fn evaluate_reviewable_call_challenge_stage<R: Rng + ?Sized>(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    period_duration_seconds: f64,
    manager_decision_inbox: &ManagerDecisionInbox,
    rng: &mut R,
) {
    if publisher.state().control_mode_for_team(team_id) == ManagerControlMode::Ai {
        if let Some((call_team_id, call)) = publisher.state().last_reviewable_call().cloned() {
            if call_team_id == team_id {
                let context = ManagerDecisionContext::build(publisher.state(), team_id);
                let challenge_cooldown = derive_cooldown_seconds(
                    period_duration_seconds,
                    context.manager_snapshot.challenge_judgment,
                );
                if publisher.state().is_decision_ready(
                    team_id,
                    ManagerDecisionKind::Challenge,
                    challenge_cooldown,
                ) {
                    if evaluate_and_execute_challenge(publisher, &context, team_id, &call, rng) {
                        publisher
                            .state_mut()
                            .mark_decision_triggered(team_id, ManagerDecisionKind::Challenge);
                    }
                }
            }
        }
    } else {
        try_apply_human_challenge(publisher, team_id, manager_decision_inbox);
    }
}
