use crate::manager_ai::play_calling::execute_play_call_selection;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;
use arlo_manager_control::ManagerDecisionInbox;
use uuid::Uuid;

pub fn try_apply_human_play_call(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    inbox: &ManagerDecisionInbox,
) -> bool {
    let Some(intent) = inbox.take_play_call(team_id) else {
        return false;
    };
    if let Some(play_call) = publisher
        .state()
        .playbook_for_team(team_id)
        .iter()
        .find(|p| p.id() == intent.play_call_id())
        .cloned()
    {
        execute_play_call_selection(publisher, team_id, play_call);
        true
    } else {
        false
    }
}