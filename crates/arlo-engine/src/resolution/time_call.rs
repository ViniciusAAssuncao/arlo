use crate::error::{EngineError, EngineResult};
use crate::input::MatchInput;
use crate::state::MatchState;
use crate::step::StepResult;
use arlo_events::{DownAdvanced, MatchEvent, TimeCallReason, TimeCallUsed};
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
        events.push(next.emit(MatchEvent::DownAdvanced(DownAdvanced::new(
            u32::from(outcome.prior_down),
            u32::from(next.series().down()),
            outcome.gain_mirim,
            outcome.total_advance_mirim,
            outcome.first_down,
            position,
        )))?);
    }
    *state = next;
    Ok(StepResult::resolved(events))
}
