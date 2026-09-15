use crate::manager_ai::challenges::apply_foul_challenge;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;
use arlo_manager_control::ManagerDecisionInbox;
use uuid::Uuid;

pub fn try_apply_human_foul_challenge(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    inbox: &ManagerDecisionInbox,
) -> bool {
    if inbox.take_challenge(team_id).is_none() {
        return false;
    }
    let Some((foul_team_id, record)) = publisher.state().last_reviewable_foul().cloned() else {
        return false;
    };
    if foul_team_id != team_id {
        return false;
    }
    apply_foul_challenge(publisher, team_id, &record)
}
