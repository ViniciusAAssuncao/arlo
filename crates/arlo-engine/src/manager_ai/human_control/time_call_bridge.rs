use crate::manager_ai::cognition::ManagerDecisionKind;
use crate::manager_ai::time_calls::execute_time_call;
use crate::time::DurationLedger;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::{EventSink, TimeCallReason};
use arlo_manager_control::ManagerDecisionInbox;
use uuid::Uuid;

pub fn try_apply_human_time_call(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    is_home: bool,
    inbox: &ManagerDecisionInbox,
) -> bool {
    if inbox.take_time_call(team_id).is_none() {
        return false;
    }
    let mut ledger = DurationLedger::new();
    if execute_time_call(
        publisher,
        team_id,
        is_home,
        &mut ledger,
        TimeCallReason::Standard,
    ) {
        publisher
            .state_mut()
            .mark_decision_triggered(team_id, ManagerDecisionKind::TimeCall);
        true
    } else {
        false
    }
}
