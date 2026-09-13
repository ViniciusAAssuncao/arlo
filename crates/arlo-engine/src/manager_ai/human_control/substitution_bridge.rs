use crate::manager_ai::cognition::ManagerDecisionKind;
use crate::manager_ai::substitutions::{execute_substitutions, SubstitutionPlan};
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::{EventSink, SubstitutionReason};
use arlo_manager_control::ManagerDecisionInbox;
use uuid::Uuid;

pub fn try_apply_human_substitutions(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    inbox: &ManagerDecisionInbox,
) -> bool {
    let intents = inbox.take_substitutions(team_id);
    if intents.is_empty() {
        return false;
    }
    let plans: Vec<SubstitutionPlan> = intents
        .into_iter()
        .map(|intent| SubstitutionPlan {
            outgoing_id: intent.outgoing_player_id(),
            incoming_id: intent.incoming_player_id(),
            reason: SubstitutionReason::Tactical,
        })
        .collect();

    match execute_substitutions(publisher, team_id, &plans) {
        Ok(executed) => {
            if executed > 0 {
                publisher
                    .state_mut()
                    .mark_decision_triggered(team_id, ManagerDecisionKind::Substitution);
                true
            } else {
                false
            }
        }
        Err(_) => false,
    }
}
