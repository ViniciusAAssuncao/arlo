mod response;
mod sample;

use crate::error::EngineResult;
use crate::input::MatchInput;
use crate::state::MatchState;
use crate::step::{StepOutcome, StepResult};
use arlo_events::MatchEvent;

pub(super) fn resolve_injuries(
    input: &MatchInput,
    state: &mut MatchState,
    result: StepResult,
) -> EngineResult<StepResult> {
    let (mut events, outcome) = result.into_parts();
    if matches!(outcome, StepOutcome::AwaitingDecision(_)) {
        return Ok(StepResult::awaiting_decision(match outcome {
            StepOutcome::AwaitingDecision(decisions) => decisions,
            _ => unreachable!(),
        }));
    }
    let sampled = events.iter().find_map(|envelope| {
        let exposure = match envelope.event() {
            MatchEvent::DuelResolved(duel) => sample::Exposure::duel(duel, state),
            MatchEvent::CarryResolved(carry) => Some(sample::Exposure::carry(carry.carrier_id())),
            _ => None,
        };
        exposure.and_then(|exposure| sample::sample_injury(input, state, exposure))
    });
    if let Some(incident) = sampled {
        response::apply_injury(input, state, &mut events, incident)?;
    }
    Ok(match outcome {
        StepOutcome::Resolved => StepResult::resolved(events),
        StepOutcome::Finished => StepResult::finished(events),
        StepOutcome::AwaitingDecision(_) => unreachable!(),
    })
}
