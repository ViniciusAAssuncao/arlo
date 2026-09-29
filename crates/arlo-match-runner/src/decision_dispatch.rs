use crate::error::{MatchRunnerError, MatchRunnerResult};
use arlo_engine::{
    resolve_forced_substitution_segment, resolve_injury_decision_segment, resolve_kick_foul_segment, resolve_next_segment, resolve_time_call_segment, MatchInput,
    MatchPhase, MatchState, StepOutcome, StepResult,
};
use arlo_manager_control::{ManagerDecisionInbox, RequiredManagerDecision};
use arlo_tactics::PlayCall;

pub fn resolve_segment(
    input: &MatchInput,
    state: &mut MatchState,
    inbox: &ManagerDecisionInbox,
    play_calls: &[PlayCall],
) -> MatchRunnerResult<StepResult> {
    if let Some(&(team_id, player_id)) = state.pending_forced_substitutions().first() {
        if let Some(intent) = inbox.forced_substitution(team_id, player_id) {
            let result = resolve_forced_substitution_segment(input, state, team_id, intent)?;
            inbox.take_forced_substitution(team_id, player_id);
            return Ok(result);
        }
        return Ok(StepResult::awaiting_decision(vec![RequiredManagerDecision::ForcedSubstitution {
            team_id, outgoing_player_ids: vec![player_id],
        }]));
    }
    if !matches!(state.phase(), MatchPhase::Live | MatchPhase::Finished) {
        if let Some(pending) = state.pending_injury_decisions().iter()
            .find(|pending| state.injury_decisions_ready() && state.injury_decision_is_actionable(**pending)) {
            let team_id = pending.team_id();
            let player_id = pending.player_id();
            if let Some(intent) = inbox.injury_decision(team_id, player_id) {
                let result = resolve_injury_decision_segment(input, state, team_id, intent)?;
                inbox.take_injury_decision(team_id, player_id);
                return Ok(result);
            }
            return Ok(StepResult::awaiting_decision(vec![RequiredManagerDecision::InjuryResponse {
                team_id, injured_player_id: player_id,
            }]));
        }
    }
    if state.phase() == MatchPhase::Live {
        let team_id = state.possessor_team_id();
        if inbox.time_call(team_id).is_some() {
            let result = resolve_time_call_segment(input, state, team_id)?;
            inbox.take_time_call(team_id);
            return Ok(result);
        }
    }
    if state.phase() == MatchPhase::KickFoul {
        let team_id = state.possessor_team_id();
        if let Some(intent) = inbox.kick_foul_decision(team_id) {
            let result = resolve_kick_foul_segment(
                input,
                state,
                Some(intent.decision()),
                intent.taker_id(),
            )?;
            if matches!(
                result.outcome(),
                StepOutcome::Resolved | StepOutcome::Finished
            ) {
                inbox.take_kick_foul_decision(team_id);
            }
            return Ok(result);
        }
    }
    if matches!(state.phase(), MatchPhase::Ready | MatchPhase::Stopped)
        && state.clock().seconds_in_period() < state.clock().period_limit_seconds()
    {
        let team_id = state.next_call_team_id();
        if let Some(intent) = inbox.play_call(team_id) {
            let call = play_calls
                .iter()
                .find(|call| call.id() == intent.play_call_id())
                .ok_or(MatchRunnerError::UnknownPlayCall {
                    play_call_id: intent.play_call_id(),
                })?;
            let result = resolve_next_segment(input, state, Some(call))?;
            if matches!(
                result.outcome(),
                StepOutcome::Resolved | StepOutcome::Finished
            ) {
                inbox.take_play_call(team_id);
            }
            return Ok(result);
        }
    }
    Ok(resolve_next_segment(input, state, None)?)
}
