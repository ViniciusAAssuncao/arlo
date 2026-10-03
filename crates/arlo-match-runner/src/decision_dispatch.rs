use crate::error::{MatchRunnerError, MatchRunnerResult};
use crate::manager_ai;
use crate::manager_play_call;
use arlo_engine::{
    resolve_forced_substitution_segment, resolve_injury_decision_segment,
    resolve_kick_foul_segment, resolve_next_segment, resolve_prepared_plan_segment,
    resolve_substitution_segment, resolve_tactical_realignment_segment,
    resolve_tactical_switch_segment, resolve_time_call_segment, select_tactical_profile,
    should_use_time_call, try_resolve_automatic_injury_decision_segment, MatchInput, MatchPhase,
    MatchState, StepOutcome, StepResult,
};
use arlo_events::SubstitutionReason;
use arlo_manager_control::{ManagerDecisionInbox, RequiredManagerDecision};
use arlo_stats::AggregatorRegistry;
use arlo_tactics::PlayCall;

pub fn resolve_segment(
    input: &MatchInput,
    state: &mut MatchState,
    inbox: &ManagerDecisionInbox,
    play_calls: &[PlayCall],
) -> MatchRunnerResult<StepResult> {
    dispatch(input, state, inbox, play_calls, None)
}

pub fn resolve_segment_with_registry(
    input: &MatchInput,
    state: &mut MatchState,
    inbox: &ManagerDecisionInbox,
    play_calls: &[PlayCall],
    registry: &AggregatorRegistry,
) -> MatchRunnerResult<StepResult> {
    dispatch(input, state, inbox, play_calls, Some(registry))
}

fn dispatch(
    input: &MatchInput,
    state: &mut MatchState,
    inbox: &ManagerDecisionInbox,
    play_calls: &[PlayCall],
    registry: Option<&AggregatorRegistry>,
) -> MatchRunnerResult<StepResult> {
    if let Some(&(team_id, player_id)) = state.pending_forced_substitutions().first() {
        if let Some(intent) = inbox.forced_substitution(team_id, player_id) {
            let result = resolve_forced_substitution_segment(input, state, team_id, intent)?;
            inbox.take_forced_substitution(team_id, player_id);
            return Ok(result);
        }
        return Ok(StepResult::awaiting_decision(vec![
            RequiredManagerDecision::ForcedSubstitution {
                team_id,
                outgoing_player_ids: vec![player_id],
            },
        ]));
    }
    if !matches!(state.phase(), MatchPhase::Live | MatchPhase::Finished) {
        if let Some(pending) = state
            .pending_injury_decisions()
            .iter()
            .copied()
            .find(|pending| {
                state.injury_decisions_ready() && state.injury_decision_is_actionable(*pending)
            })
        {
            let team_id = pending.team_id();
            let player_id = pending.player_id();
            if let Some(result) =
                try_resolve_automatic_injury_decision_segment(input, state, pending)?
            {
                return Ok(result);
            }
            if let Some(intent) = inbox.injury_decision(team_id, player_id) {
                let result = resolve_injury_decision_segment(input, state, team_id, intent)?;
                inbox.take_injury_decision(team_id, player_id);
                return Ok(result);
            }
            return Ok(StepResult::awaiting_decision(vec![
                RequiredManagerDecision::InjuryResponse {
                    team_id,
                    injured_player_id: player_id,
                },
            ]));
        }
    }
    if state.phase() == MatchPhase::Live {
        let team_id = state.possessor_team_id();
        if inbox.time_call(team_id).is_some() {
            let result = resolve_time_call_segment(input, state, team_id)?;
            inbox.take_time_call(team_id);
            return Ok(result);
        }
        if should_use_time_call(input, state) {
            return Ok(resolve_time_call_segment(input, state, team_id)?);
        }
    }
    if state.phase() == MatchPhase::Stopped {
        for team_id in [input.home().team_id(), input.away().team_id()] {
            if let Some(intent) = inbox.tactical_switch(team_id) {
                let result =
                    resolve_tactical_switch_segment(input, state, team_id, intent.profile_id())?;
                inbox.take_tactical_switch(team_id);
                return Ok(result);
            }
            if let Some(intent) = inbox.prepared_plan(team_id) {
                let result = resolve_prepared_plan_segment(input, state, team_id, &intent)?;
                inbox.take_prepared_plan(team_id);
                return Ok(result);
            }
            if let Some(intent) = inbox.tactical_realignment(team_id) {
                let result = resolve_tactical_realignment_segment(input, state, team_id, &intent)?;
                inbox.take_tactical_realignment(team_id);
                return Ok(result);
            }
            let team = if team_id == input.home().team_id() {
                input.home()
            } else {
                input.away()
            };
            if let Some(profile_id) = team
                .prepared_plans()
                .is_empty()
                .then(|| select_tactical_profile(input, state, team_id))
                .flatten()
            {
                return Ok(resolve_tactical_switch_segment(
                    input, state, team_id, profile_id,
                )?);
            }
            let submitted = inbox.substitutions(team_id);
            if !submitted.is_empty() {
                let result = resolve_substitution_segment(
                    input,
                    state,
                    team_id,
                    &submitted,
                    SubstitutionReason::Tactical,
                )?;
                inbox.take_substitutions(team_id);
                return Ok(result);
            }
            if let Some(action) = manager_ai::select_action(input, state, registry, team_id) {
                return Ok(match action {
                    manager_ai::ManagerAction::PreparedPlan(intent) => {
                        resolve_prepared_plan_segment(input, state, team_id, &intent)?
                    }
                    manager_ai::ManagerAction::Realignment(intent) => {
                        resolve_tactical_realignment_segment(input, state, team_id, &intent)?
                    }
                    manager_ai::ManagerAction::Substitution(intent, reason) => {
                        resolve_substitution_segment(input, state, team_id, &[intent], reason)?
                    }
                });
            }
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
    let call = if matches!(state.phase(), MatchPhase::Ready | MatchPhase::Stopped) {
        manager_play_call::select(input, state, play_calls)
    } else {
        None
    };
    Ok(resolve_next_segment(input, state, call)?)
}
