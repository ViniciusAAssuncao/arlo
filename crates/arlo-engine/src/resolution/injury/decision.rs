use crate::error::{EngineError, EngineResult};
use crate::input::MatchInput;
use crate::state::{MatchPhase, MatchState};
use crate::step::StepResult;
use arlo_events::{AvailabilityStatus, MatchEvent, PlayerAvailabilityChanged, SubstitutionMade, SubstitutionReason};
use arlo_manager_control::{ForcedSubstitutionIntent, InjuryDecisionIntent};
use uuid::Uuid;

pub fn resolve_injury_decision_segment(
    input: &MatchInput,
    state: &mut MatchState,
    team_id: Uuid,
    intent: InjuryDecisionIntent,
) -> EngineResult<StepResult> {
    super::super::context::validate_match_state(input, state)?;
    if !matches!(state.phase(), MatchPhase::Ready | MatchPhase::Stopped | MatchPhase::PeriodBreak)
        || !state.injury_decisions_ready()
        || !state.pending_injury_decisions().contains(&(team_id, intent.injured_player_id()))
    {
        return Err(EngineError::InvalidTransition("injury decision requires a pending player at a stoppage".into()));
    }
    let team = if team_id == input.home().team_id() {
        state.home()
    } else if team_id == input.away().team_id() {
        state.away()
    } else {
        return Err(EngineError::InvalidInput("unknown injury decision team".into()));
    };
    if !team.active_player_ids().contains(&intent.injured_player_id())
        || !team.injured_player_ids().contains(&intent.injured_player_id())
    {
        return Err(EngineError::InvalidTransition("injury decision player is not active and injured".into()));
    }
    if let Some(replacement_id) = intent.replacement_player_id() {
        if !team.reserve_player_ids().contains(&replacement_id) {
            return Err(EngineError::InvalidInput("injury replacement is not an available reserve".into()));
        }
    }
    let mut next = state.clone();
    let mut events = Vec::new();
    if intent.withdraws_player() {
        next.withdraw_injured_player(team_id, intent.injured_player_id(), intent.replacement_player_id())?;
        events.push(next.emit(MatchEvent::PlayerAvailabilityChanged(PlayerAvailabilityChanged::new(
            intent.injured_player_id(), team_id, AvailabilityStatus::Active,
            AvailabilityStatus::Injured, None,
        )))?);
        if let Some(replacement_id) = intent.replacement_player_id() {
            let clock = arlo_events::MatchClockInstant::with_total_elapsed_seconds(
                next.clock().period(), next.clock().seconds_in_period(), next.clock().total_elapsed_seconds(),
            );
            events.push(next.emit(MatchEvent::SubstitutionMade(SubstitutionMade::new(
                team_id, intent.injured_player_id(), replacement_id, clock, SubstitutionReason::Injury,
            )))?);
        }
    } else {
        next.clear_injury_decision(team_id, intent.injured_player_id());
    }
    *state = next;
    Ok(StepResult::resolved(events))
}

pub fn resolve_forced_substitution_segment(
    input: &MatchInput,
    state: &mut MatchState,
    team_id: Uuid,
    intent: ForcedSubstitutionIntent,
) -> EngineResult<StepResult> {
    super::super::context::validate_match_state(input, state)?;
    if !state.pending_forced_substitutions().contains(&(team_id, intent.outgoing_player_id()))
    {
        return Err(EngineError::InvalidTransition("forced substitution requires a pending injured player".into()));
    }
    let team = if team_id == input.home().team_id() {
        state.home()
    } else if team_id == input.away().team_id() {
        state.away()
    } else {
        return Err(EngineError::InvalidInput("unknown forced substitution team".into()));
    };
    if team.active_player_ids().contains(&intent.outgoing_player_id())
        || !team.injured_player_ids().contains(&intent.outgoing_player_id())
        || !team.reserve_player_ids().contains(&intent.incoming_player_id())
    {
        return Err(EngineError::InvalidInput("forced substitution requires an injured outgoing player and an available reserve".into()));
    }
    let mut next = state.clone();
    next.replace_withdrawn_player(team_id, intent.outgoing_player_id(), intent.incoming_player_id())?;
    let clock = arlo_events::MatchClockInstant::with_total_elapsed_seconds(
        next.clock().period(), next.clock().seconds_in_period(), next.clock().total_elapsed_seconds(),
    );
    let event = next.emit(MatchEvent::SubstitutionMade(SubstitutionMade::new(
        team_id, intent.outgoing_player_id(), intent.incoming_player_id(), clock, SubstitutionReason::Injury,
    )))?;
    *state = next;
    Ok(StepResult::resolved(vec![event]))
}
