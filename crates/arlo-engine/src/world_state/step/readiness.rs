use crate::manager_ai::human_control::try_apply_human_play_call;
use crate::manager_ai::substitutions::resolve_forced_substitutions_for_team;
use crate::world_state::match_state::MatchState;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_domain::ManagerControlMode;
use arlo_events::EventSink;
use arlo_manager_control::{ManagerDecisionInbox, RequiredManagerDecision};

pub fn peek_pending_manager_decisions(state: &MatchState) -> Vec<RequiredManagerDecision> {
    let home_id = state.home_team_id();
    let away_id = state.away_team_id();
    let mut decisions = Vec::new();

    for team_id in [home_id, away_id] {
        if state.control_mode_for_team(team_id) != ManagerControlMode::Ai {
            let pending_ids = state.pending_forced_substitutions_for(team_id);
            if !pending_ids.is_empty() {
                decisions.push(RequiredManagerDecision::ForcedSubstitution {
                    team_id,
                    outgoing_player_ids: pending_ids.to_vec(),
                });
            }
        }
    }

    if !decisions.is_empty() {
        return decisions;
    }

    let offense_id = state.possession().offense();
    if state.control_mode_for_team(offense_id) != ManagerControlMode::Ai
        && !state.has_queued_call_for_offense()
    {
        decisions.push(RequiredManagerDecision::PlayCall { team_id: offense_id });
    }

    decisions
}

pub(crate) fn resolve_pending_manager_decisions(
    state: &mut MatchState,
    sink: &mut impl EventSink,
    inbox: &ManagerDecisionInbox,
) -> Vec<RequiredManagerDecision> {
    let home_id = state.home_team_id();
    let away_id = state.away_team_id();
    let mut decisions = Vec::new();

    {
        let mut publisher = EventPublisher::new(state, sink);
        for team_id in [home_id, away_id] {
            if publisher.state().control_mode_for_team(team_id) != ManagerControlMode::Ai
                && !publisher.state().pending_forced_substitutions_for(team_id).is_empty()
            {
                let remaining_ids =
                    resolve_forced_substitutions_for_team(&mut publisher, team_id, inbox);
                if !remaining_ids.is_empty() {
                    decisions.push(RequiredManagerDecision::ForcedSubstitution {
                        team_id,
                        outgoing_player_ids: remaining_ids,
                    });
                }
            }
        }
    }

    if !decisions.is_empty() {
        return decisions;
    }

    let offense_id = state.possession().offense();
    if state.control_mode_for_team(offense_id) != ManagerControlMode::Ai
        && !state.has_queued_call_for_offense()
    {
        let mut publisher = EventPublisher::new(state, sink);
        try_apply_human_play_call(&mut publisher, offense_id, inbox);
        if !publisher.state().has_queued_call_for_offense() {
            decisions.push(RequiredManagerDecision::PlayCall { team_id: offense_id });
        }
    }

    decisions
}