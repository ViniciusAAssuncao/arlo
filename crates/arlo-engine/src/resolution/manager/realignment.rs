use crate::error::{EngineError, EngineResult};
use crate::input::MatchInput;
use crate::resolution::context::validate_match_state;
use crate::state::{MatchPhase, MatchState};
use crate::step::StepResult;
use arlo_events::{MatchEvent, TacticalAssignment, TacticalRealignmentMade};
use arlo_manager_control::TacticalRealignmentIntent;
use uuid::Uuid;

pub fn resolve_tactical_realignment_segment(
    input: &MatchInput,
    state: &mut MatchState,
    team_id: Uuid,
    intent: &TacticalRealignmentIntent,
) -> EngineResult<StepResult> {
    validate_match_state(input, state)?;
    if state.phase() != MatchPhase::Stopped
        || state.clock().seconds_in_period() >= state.clock().period_limit_seconds()
    {
        return Err(EngineError::InvalidTransition(
            "realignment requires a stoppage before the next play".into(),
        ));
    }
    let team = if team_id == input.home().team_id() {
        input.home()
    } else if team_id == input.away().team_id() {
        input.away()
    } else {
        return Err(EngineError::InvalidInput("unknown realignment team".into()));
    };
    let current = if team_id == input.home().team_id() {
        state.home()
    } else {
        state.away()
    };
    let first = intent.first_player_id();
    let second = intent.second_player_id();
    if first == second
        || !current.active_player_ids().contains(&first)
        || !current.active_player_ids().contains(&second)
    {
        return Err(EngineError::InvalidInput(
            "realignment requires two distinct active players".into(),
        ));
    }
    let locate = |player_id| {
        let mut assignments = current
            .lineup(team)
            .assignments()
            .iter()
            .filter(|assignment| current.slot_player_id(assignment.player_id()) == player_id);
        let assignment = assignments.next()?;
        if assignments.next().is_some() {
            None
        } else {
            Some(assignment)
        }
    };
    let first_slot = locate(first)
        .ok_or_else(|| EngineError::InvalidInput("invalid first realignment slot".into()))?;
    let second_slot = locate(second)
        .ok_or_else(|| EngineError::InvalidInput("invalid second realignment slot".into()))?;
    let describe = |assignment: &arlo_tactics::SlotAssignment,
                    player_id|
     -> EngineResult<TacticalAssignment> {
        let slot = current
            .formation(team)
            .slots()
            .get(assignment.formation_slot_index())
            .ok_or_else(|| EngineError::InvalidInput("unknown formation slot".into()))?;
        Ok(TacticalAssignment {
            player_id,
            formation_slot_index: assignment.formation_slot_index(),
            offensive_position: slot.offensive_position(),
            defensive_position: slot.defensive_position(),
            slot_role: assignment.slot_role(),
        })
    };
    let assignments = [describe(second_slot, first)?, describe(first_slot, second)?];
    let mut next = state.clone();
    next.realign_slots(team_id, first_slot.player_id(), second_slot.player_id())?;
    let event = next.emit(MatchEvent::TacticalRealignmentMade(
        TacticalRealignmentMade::new(team_id, assignments),
    ))?;
    *state = next;
    Ok(StepResult::resolved(vec![event]))
}
