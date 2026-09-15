use crate::manager_ai::cognition::ManagerDecisionKind;
use crate::manager_ai::tactical_adjustment::execute_tactical_adjustment_by_id;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;
use arlo_manager_control::ManagerDecisionInbox;
use uuid::Uuid;

pub fn try_apply_human_tactical_switch(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    inbox: &ManagerDecisionInbox,
) -> bool {
    let Some(intent) = inbox.take_tactical_switch(team_id) else {
        return false;
    };
    let available_profiles = publisher
        .state()
        .available_profiles_for_team(team_id)
        .to_vec();
    if execute_tactical_adjustment_by_id(
        publisher,
        team_id,
        intent.profile_id(),
        &available_profiles,
    ) {
        publisher
            .state_mut()
            .mark_decision_triggered(team_id, ManagerDecisionKind::TacticalAdjustment);
        true
    } else {
        false
    }
}
