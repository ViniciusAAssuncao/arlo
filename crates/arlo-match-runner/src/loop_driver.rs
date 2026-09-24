use crate::decision_dispatch::resolve_segment;
use crate::dual_sink::DualEventSink;
use crate::error::{MatchRunnerError, MatchRunnerResult};
use crate::match_run_result::MatchRunResult;
use crate::match_run_status::MatchRunStatus;
use arlo_engine::{MatchInput, MatchPhase, MatchState, StepOutcome};
use arlo_events::{EventSink, InMemorySink};
use arlo_manager_control::ManagerDecisionInbox;
use arlo_stats::AggregatorRegistry;
use arlo_tactics::PlayCall;

pub const DEFAULT_MAX_SEGMENTS: usize = 100_000;

pub fn run_match(input: &MatchInput, state: &mut MatchState) -> MatchRunnerResult<MatchRunResult> {
    run_match_with_limit(input, state, DEFAULT_MAX_SEGMENTS)
}

pub fn run_match_with_limit(
    input: &MatchInput,
    state: &mut MatchState,
    max_segments: usize,
) -> MatchRunnerResult<MatchRunResult> {
    run_match_with_registry(
        input,
        state,
        AggregatorRegistry::with_default_aggregators(),
        max_segments,
    )
}

pub fn run_match_with_inbox(
    input: &MatchInput,
    state: &mut MatchState,
    inbox: &ManagerDecisionInbox,
    play_calls: &[PlayCall],
) -> MatchRunnerResult<MatchRunStatus> {
    let mut raw_sink = InMemorySink::new();
    let mut aggregators = AggregatorRegistry::with_default_aggregators();
    let outcome = run_loop_with_inbox(
        input,
        state,
        inbox,
        play_calls,
        &mut raw_sink,
        &mut aggregators,
        DEFAULT_MAX_SEGMENTS,
    );
    let result = MatchRunResult::new(raw_sink, aggregators);
    match outcome {
        Ok(_) => Ok(MatchRunStatus::Finished(result)),
        Err(MatchRunnerError::AwaitingManagerDecision { decisions }) => {
            Ok(MatchRunStatus::AwaitingDecision {
                decisions,
                partial: result,
            })
        }
        Err(error) => Err(error),
    }
}

pub fn run_match_with_registry(
    input: &MatchInput,
    state: &mut MatchState,
    mut aggregators: AggregatorRegistry,
    max_segments: usize,
) -> MatchRunnerResult<MatchRunResult> {
    let mut raw_sink = InMemorySink::new();
    run_loop(input, state, &mut raw_sink, &mut aggregators, max_segments)?;
    Ok(MatchRunResult::new(raw_sink, aggregators))
}

pub fn run_loop(
    input: &MatchInput,
    state: &mut MatchState,
    raw_sink: &mut InMemorySink,
    aggregators: &mut AggregatorRegistry,
    max_segments: usize,
) -> MatchRunnerResult<usize> {
    let inbox = ManagerDecisionInbox::new();
    run_loop_with_inbox(
        input,
        state,
        &inbox,
        &[],
        raw_sink,
        aggregators,
        max_segments,
    )
}

pub fn run_loop_with_inbox(
    input: &MatchInput,
    state: &mut MatchState,
    inbox: &ManagerDecisionInbox,
    play_calls: &[PlayCall],
    raw_sink: &mut InMemorySink,
    aggregators: &mut AggregatorRegistry,
    max_segments: usize,
) -> MatchRunnerResult<usize> {
    let mut segments = 0;
    let mut dual_sink = DualEventSink::new(raw_sink, aggregators);
    while state.phase() != MatchPhase::Finished {
        if segments >= max_segments {
            return Err(MatchRunnerError::SegmentLimitExceeded { max_segments });
        }
        let result = resolve_segment(input, state, inbox, play_calls)?;
        let (events, outcome) = result.into_parts();
        dual_sink.record_all(events);
        match outcome {
            StepOutcome::Resolved => segments += 1,
            StepOutcome::Finished => {
                segments += 1;
                break;
            }
            StepOutcome::AwaitingDecision(decisions) => {
                return Err(MatchRunnerError::AwaitingManagerDecision { decisions });
            }
        }
    }
    Ok(segments)
}
