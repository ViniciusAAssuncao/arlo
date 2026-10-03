use super::AuditResult;
use arlo_engine::{MatchInput, MatchPhase, MatchState, StepOutcome};
use arlo_events::{EventSink, InMemorySink, MatchEvent};
use arlo_manager_control::ManagerDecisionInbox;
use arlo_match_runner::{
    build_default_aggregator_registry, manager_ai::assess_manager_decision,
    resolve_segment_with_registry, DualEventSink, DEFAULT_MAX_SEGMENTS,
};
use arlo_tactics::PlayCall;

pub fn audit(input: &MatchInput, calls: &[PlayCall]) -> AuditResult<()> {
    let misaligned = super::scenario_input::misaligned(input)?;
    let first = simulate(&misaligned, calls)?;
    if first != simulate(&misaligned, calls)? {
        return Err("Misalignment behavior is not reproducible".into());
    }
    println!("Behavior: AI corrected misplaced starters; attributes changed eligible actions; identical replay");
    Ok(())
}

fn simulate(
    input: &MatchInput,
    calls: &[PlayCall],
) -> AuditResult<Vec<arlo_events::MatchEventEnvelope>> {
    let misaligned = input;
    let mut state = MatchState::new(&misaligned);
    let mut registry = build_default_aggregator_registry(&misaligned)?;
    let inbox = ManagerDecisionInbox::new();
    let mut raw = InMemorySink::new();
    let low = super::scenario_input::manager(&misaligned, 1)?;
    let high = super::scenario_input::manager(&misaligned, 20)?;
    let mut adjusted = false;
    let mut differentiated = false;
    for _ in 0..DEFAULT_MAX_SEGMENTS {
        if state.phase() == MatchPhase::Stopped {
            let a = assess_manager_decision(&low, &state, &registry, input.home().team_id());
            let b = assess_manager_decision(&high, &state, &registry, input.home().team_id());
            if let (Some(a), Some(b)) = (a, b) {
                differentiated |= a.realignments.len() != b.realignments.len();
            }
        }
        let (events, outcome) =
            resolve_segment_with_registry(&misaligned, &mut state, &inbox, calls, &registry)?
                .into_parts();
        let mut sink = DualEventSink::new(&mut raw, &mut registry);
        for event in events {
            if matches!(event.event(), MatchEvent::TacticalRealignmentMade(value) if value.team_id() == input.home().team_id())
            {
                adjusted = true;
            }
            if let MatchEvent::PlayInvalidated(value) = event.event() {
                sink.invalidate_play(value.first_sequence(), value.last_sequence());
            }
            sink.record(event);
        }
        if adjusted && differentiated {
            break;
        }
        if !matches!(outcome, StepOutcome::Resolved) {
            break;
        }
    }
    if !adjusted || !differentiated {
        return Err(format!("Misalignment behavior missing: adjusted={adjusted}, attribute contrast={differentiated}").into());
    }
    super::roster::verify(&misaligned, &state, &registry)?;
    Ok(raw.events().to_vec())
}
