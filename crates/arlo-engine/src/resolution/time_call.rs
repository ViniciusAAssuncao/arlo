use super::down::emit_down_advanced;
use crate::error::{EngineError, EngineResult};
use crate::input::MatchInput;
use crate::state::MatchState;
use crate::step::StepResult;
use arlo_events::{MatchEvent, TimeCallReason, TimeCallUsed};
use uuid::Uuid;

pub fn resolve_time_call_segment(
    input: &MatchInput,
    state: &mut MatchState,
    requesting_team_id: Uuid,
) -> EngineResult<StepResult> {
    if input.match_id() != state.match_id()
        || input.home().team_id() != state.home().team_id()
        || input.away().team_id() != state.away().team_id()
    {
        return Err(EngineError::InvalidInput(
            "state and match input differ".into(),
        ));
    }
    let mut next = state.clone();
    let pending = next.pending_call_outcome();
    let position = next.possession().ball_position_mirim();
    let (_, remaining) =
        next.resolve_time_call(requesting_team_id, input.format().time_calls_per_period())?;
    let mut events = vec![next.emit(MatchEvent::TimeCallUsed(TimeCallUsed::new(
        requesting_team_id,
        remaining,
        TimeCallReason::Standard,
    )))?];
    if let Some(outcome) = pending {
        emit_down_advanced(&mut next, &mut events, outcome, position)?;
    }
    *state = next;
    Ok(StepResult::resolved(events))
}
