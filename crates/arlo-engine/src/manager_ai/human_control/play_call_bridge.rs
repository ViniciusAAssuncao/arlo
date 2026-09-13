use crate::world_state::match_state::MatchState;
use arlo_manager_control::ManagerDecisionInbox;
use uuid::Uuid;

pub fn try_apply_human_play_call(
    state: &mut MatchState,
    team_id: Uuid,
    inbox: &ManagerDecisionInbox,
) -> bool {
    let Some(intent) = inbox.take_play_call(team_id) else {
        return false;
    };
    if let Some(play_call) = state
        .playbook_for_team(team_id)
        .iter()
        .find(|p| p.id() == intent.play_call_id())
        .cloned()
    {
        state.set_active_play_call(team_id, play_call);
        true
    } else {
        false
    }
}
