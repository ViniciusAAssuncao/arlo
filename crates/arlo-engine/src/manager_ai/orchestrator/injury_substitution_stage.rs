use crate::manager_ai::substitutions::{
    execute_forced_injury_substitutions, resolve_forced_substitutions_for_team,
};
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_domain::ManagerControlMode;
use arlo_events::EventSink;
use arlo_manager_control::ManagerDecisionInbox;
use uuid::Uuid;

pub fn evaluate_injury_substitution_stage(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    manager_decision_inbox: &ManagerDecisionInbox,
) {
    if publisher.state().control_mode_for_team(team_id) == ManagerControlMode::Ai {
        execute_forced_injury_substitutions(publisher, team_id);
        publisher
            .state_mut()
            .clear_pending_forced_substitutions(team_id);
    } else {
        resolve_forced_substitutions_for_team(publisher, team_id, manager_decision_inbox);
    }
}