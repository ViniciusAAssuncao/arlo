use crate::error::EngineResult;
use crate::state::{MatchState, PendingCallOutcome};
use arlo_events::{DownAdvanced, MatchEvent, MatchEventEnvelope};

pub(super) fn emit_down_advanced(
    state: &mut MatchState,
    events: &mut Vec<MatchEventEnvelope>,
    outcome: PendingCallOutcome,
    position_mirim: f64,
) -> EngineResult<()> {
    events.push(state.emit(MatchEvent::DownAdvanced(DownAdvanced::new(
        u32::from(outcome.prior_down),
        u32::from(state.series().down()),
        outcome.gain_mirim,
        outcome.total_advance_mirim,
        outcome.first_down,
        position_mirim,
    )))?);
    Ok(())
}
