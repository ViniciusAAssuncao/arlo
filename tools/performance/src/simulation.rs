use crate::Result;
use arlo_analytics::PlayerPerformanceAggregator;
use arlo_engine::{MatchInput, MatchPhase, MatchState, StepOutcome};
use arlo_events::{EventSink, InMemorySink, MatchEvent};
use arlo_manager_control::ManagerDecisionInbox;
#[cfg(not(feature = "pre-epic"))]
use arlo_match_runner::resolve_segment_with_registry;
use arlo_match_runner::{
    build_default_aggregator_registry, run_match_with_registry_and_play_calls, DualEventSink,
    MatchRunResult, DEFAULT_MAX_SEGMENTS,
};
use arlo_tactics::PlayCall;
use serde::Serialize;
use std::time::Instant;

#[cfg(feature = "pre-epic")]
fn resolve_segment_with_registry(
    input: &MatchInput,
    state: &mut MatchState,
    inbox: &ManagerDecisionInbox,
    calls: &[PlayCall],
    _: &arlo_stats::AggregatorRegistry,
) -> arlo_match_runner::MatchRunnerResult<arlo_engine::StepResult> {
    arlo_match_runner::resolve_segment(input, state, inbox, calls)
}

#[derive(Default, Serialize)]
pub struct Timings {
    pub dispatch_ms: f64,
    pub live_dispatch_ms: f64,
    pub stopped_dispatch_ms: f64,
    pub other_dispatch_ms: f64,
    pub aggregation_ms: f64,
    pub invalidation_ms: f64,
    pub assessment_ms: f64,
    pub assessments: usize,
    pub segments: usize,
    pub events: usize,
    pub substitutions: usize,
    pub plans: usize,
    pub realignments: usize,
    pub invalidations: usize,
}

pub fn run(input: &MatchInput, calls: &[PlayCall]) -> Result<(MatchState, MatchRunResult)> {
    let mut state = MatchState::new(input);
    let result = run_match_with_registry_and_play_calls(
        input,
        &mut state,
        build_default_aggregator_registry(input)?,
        calls,
        DEFAULT_MAX_SEGMENTS,
    )?;
    Ok((state, result))
}

pub fn profile(
    input: &MatchInput,
    calls: &[PlayCall],
) -> Result<(Timings, Vec<serde_json::Value>)> {
    let mut state = MatchState::new(input);
    let mut registry = build_default_aggregator_registry(input)?;
    let mut raw = InMemorySink::new();
    let inbox = ManagerDecisionInbox::new();
    let mut timings = Timings::default();
    #[cfg(not(feature = "pre-epic"))]
    let mut assessments = Vec::new();
    #[cfg(feature = "pre-epic")]
    let assessments = Vec::new();
    while state.phase() != MatchPhase::Finished {
        if timings.segments >= DEFAULT_MAX_SEGMENTS {
            return Err("Segment limit".into());
        }
        #[cfg(not(feature = "pre-epic"))]
        if state.phase() == MatchPhase::Stopped {
            for id in [input.home().team_id(), input.away().team_id()] {
                let started = Instant::now();
                let assessment =
                    std::hint::black_box(arlo_match_runner::manager_ai::assess_manager_decision(
                        input, &state, &registry, id,
                    ));
                timings.assessment_ms += started.elapsed().as_secs_f64() * 1000.0;
                timings.assessments += 1;
                assessments.push(crate::comparison::assessment(assessment.as_ref()));
            }
        }
        let phase = state.phase();
        let started = Instant::now();
        let result = resolve_segment_with_registry(input, &mut state, &inbox, calls, &registry)?;
        let elapsed = started.elapsed().as_secs_f64() * 1000.0;
        timings.dispatch_ms += elapsed;
        match phase {
            MatchPhase::Live => timings.live_dispatch_ms += elapsed,
            MatchPhase::Stopped => timings.stopped_dispatch_ms += elapsed,
            _ => timings.other_dispatch_ms += elapsed,
        }
        let (events, outcome) = result.into_parts();
        for event in events {
            timings.events += 1;
            match event.event() {
                MatchEvent::SubstitutionMade(_) => timings.substitutions += 1,
                #[cfg(not(feature = "pre-epic"))]
                MatchEvent::TacticalPlanActivated(_) => timings.plans += 1,
                #[cfg(not(feature = "pre-epic"))]
                MatchEvent::TacticalRealignmentMade(_) => timings.realignments += 1,
                _ => {}
            }
            let mut sink = DualEventSink::new(&mut raw, &mut registry);
            if let MatchEvent::PlayInvalidated(value) = event.event() {
                timings.invalidations += 1;
                let started = Instant::now();
                sink.invalidate_play(value.first_sequence(), value.last_sequence());
                timings.invalidation_ms += started.elapsed().as_secs_f64() * 1000.0;
            }
            let started = Instant::now();
            sink.record(event);
            timings.aggregation_ms += started.elapsed().as_secs_f64() * 1000.0;
        }
        if matches!(outcome, StepOutcome::AwaitingDecision(_)) {
            return Err("Human decision".into());
        }
        timings.segments += 1;
    }
    Ok((timings, assessments))
}

pub fn semantic_result(state: &MatchState, run: &MatchRunResult) -> Result<serde_json::Value> {
    let analytics = run
        .aggregators
        .get::<PlayerPerformanceAggregator>()
        .ok_or("No analytics")?;
    Ok(serde_json::json!({
        "events_sha256": crate::comparison::fingerprint(&serde_json::to_value(run.raw_sink().events())?),
        "events_count": run.raw_sink().events().len(),
        "performance": analytics.all_player_snapshots(),
        "performance_states": analytics.players(),
        "assignments": analytics.context().map(|context| context.all_assignments()),
        "substitution_history": analytics.context().map(|context| context.substitution_history()),
        "player_stats": run.aggregators.player_snapshots_vec(),
        "team_stats": run.aggregators.team_snapshots_vec(),
        "home_score": state.home().score().total_points(),
        "away_score": state.away().score().total_points(),
        "clock": [f64::from(state.clock().period()),state.clock().seconds_in_period(),state.clock().total_elapsed_seconds()],
        "home_active": state.home().active_player_ids(),
        "away_active": state.away().active_player_ids(),
        "home_reserves": state.home().reserve_player_ids(),
        "away_reserves": state.away().reserve_player_ids(),
    }))
}
