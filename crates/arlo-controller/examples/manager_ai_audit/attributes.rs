use super::AuditResult;
use arlo_domain::AttributeKey;
use arlo_engine::{MatchInput, MatchPhase, MatchState, StepOutcome};
use arlo_events::{EventSink, InMemorySink, MatchEvent};
use arlo_manager_control::ManagerDecisionInbox;
use arlo_match_runner::{
    build_default_aggregator_registry, manager_ai::assess_manager_decision,
    resolve_segment_with_registry, DualEventSink, DEFAULT_MAX_SEGMENTS,
};
use arlo_tactics::PlayCall;

pub fn audit(input: &MatchInput, calls: &[PlayCall]) -> AuditResult<()> {
    let input = super::scenario_input::manager(input, 20)?;
    let keys = [
        AttributeKey::JudgingAbility,
        AttributeKey::InGameAdjustments,
        AttributeKey::TacticalKnowledge,
        AttributeKey::Adaptability,
        AttributeKey::OffensePlanning,
        AttributeKey::DefenseOrganization,
        AttributeKey::ArtroStrategy,
        AttributeKey::LoadManagement,
        AttributeKey::Composure,
        AttributeKey::Discipline,
    ];
    let variants: Vec<_> = keys
        .iter()
        .map(|key| super::scenario_input::attribute(&input, *key, 1))
        .collect::<Result<_, _>>()?;
    let mut changed = [false; 10];
    let mut state = MatchState::new(&input);
    let mut registry = build_default_aggregator_registry(&input)?;
    let mut raw = InMemorySink::new();
    let inbox = ManagerDecisionInbox::new();
    for _ in 0..DEFAULT_MAX_SEGMENTS {
        if state.phase() == MatchPhase::Stopped
            && state.clock().seconds_in_period() < state.clock().period_limit_seconds()
        {
            let baseline =
                assess_manager_decision(&input, &state, &registry, input.home().team_id())
                    .ok_or("Missing reference assessment")?;
            let expected = format!("{baseline:?}");
            for (index, variant) in variants.iter().enumerate() {
                if changed[index] {
                    continue;
                }
                let assessment =
                    assess_manager_decision(variant, &state, &registry, input.home().team_id())
                        .ok_or("Missing attribute assessment")?;
                changed[index] = format!("{assessment:?}") != expected;
                if format!("{assessment:?}")
                    != format!(
                        "{:?}",
                        assess_manager_decision(variant, &state, &registry, input.home().team_id())
                            .unwrap()
                    )
                {
                    return Err("Attribute perception is not stable for the same evidence".into());
                }
                super::lifecycle::Lifecycle::default().assessment(
                    &state,
                    input.home().team_id(),
                    &assessment,
                )?;
            }
            if changed.iter().all(|value| *value) {
                println!("Attributes: all 10 supported cognitive responsibilities affect assessment; fixed evidence and repeated perception agree; availability preserved");
                return Ok(());
            }
        }
        let (events, outcome) =
            resolve_segment_with_registry(&input, &mut state, &inbox, calls, &registry)?
                .into_parts();
        let mut sink = DualEventSink::new(&mut raw, &mut registry);
        for event in events {
            if let MatchEvent::PlayInvalidated(value) = event.event() {
                sink.invalidate_play(value.first_sequence(), value.last_sequence());
            }
            sink.record(event);
        }
        if !matches!(outcome, StepOutcome::Resolved) || state.phase() == MatchPhase::Finished {
            break;
        }
    }
    let missing: Vec<_> = keys
        .iter()
        .zip(changed)
        .filter_map(|(key, found)| (!found).then_some(key))
        .collect();
    Err(format!("Attribute audit found inactive responsibilities: {missing:?}").into())
}
