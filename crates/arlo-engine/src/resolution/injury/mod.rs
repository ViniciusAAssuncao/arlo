mod decision;
mod manager_ai;
mod response;
mod sample;

pub use decision::{resolve_forced_substitution_segment, resolve_injury_decision_segment};

use crate::error::{EngineError, EngineResult};
use crate::input::MatchInput;
use crate::state::{MatchState, PendingInjuryDecision};
use crate::step::{StepOutcome, StepResult};
use arlo_events::MatchEvent;

pub fn try_resolve_automatic_injury_decision_segment(
    input: &MatchInput,
    state: &mut MatchState,
    pending: PendingInjuryDecision,
) -> EngineResult<Option<StepResult>> {
    let team = if pending.team_id() == input.home().team_id() {
        input.home()
    } else if pending.team_id() == input.away().team_id() {
        input.away()
    } else {
        return Err(EngineError::InvalidInput(
            "unknown injury decision team".into(),
        ));
    };
    if team.manager().is_human_controlled() {
        return Ok(None);
    }
    let intent = manager_ai::decide(input, state, pending)?;
    Ok(Some(decision::resolve_injury_decision_segment(
        input,
        state,
        pending.team_id(),
        intent,
    )?))
}

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
            MatchEvent::PasserContactResolved(contact) => sample::Exposure::passer_contact(contact),
            MatchEvent::CarryResolved(carry) => Some(sample::Exposure::carry(carry.carrier_id())),
            _ => None,
        };
        exposure.and_then(|exposure| sample::sample_injury(input, state, exposure))
    });
    if let Some(incident) = sampled {
        response::apply_injury(input, state, &mut events, incident)?;
    }
    if events.iter().any(|envelope| matches!(envelope.event(), MatchEvent::OutOfBounds(_))) {
        state.mark_injury_out();
    }
    if state.injury_decisions_ready()
        && !matches!(
            state.phase(),
            crate::state::MatchPhase::Live | crate::state::MatchPhase::Finished
        )
    {
        let pending: Vec<_> = state
            .pending_injury_decisions()
            .iter()
            .copied()
            .filter(|pending| state.injury_decision_is_actionable(*pending))
            .collect();
        for injury in pending {
            if let Some(result) =
                try_resolve_automatic_injury_decision_segment(input, state, injury)?
            {
                let (decision_events, _) = result.into_parts();
                events.extend(decision_events);
            }
        }
    }
    Ok(match outcome {
        StepOutcome::Resolved => StepResult::resolved(events),
        StepOutcome::Finished => StepResult::finished(events),
        StepOutcome::AwaitingDecision(_) => unreachable!(),
    })
}
