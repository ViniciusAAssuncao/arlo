use crate::dual_sink::DualEventSink;
use crate::error::{MatchRunnerError, MatchRunnerResult};
use crate::match_run_result::MatchRunResult;
use arlo_engine::{step_call_to_action, MatchState};
use arlo_events::InMemorySink;
use arlo_stats::AggregatorRegistry;
use rand::Rng;

pub const DEFAULT_MAX_ITERATIONS: usize = 100_000;

pub fn run_match<R: Rng + ?Sized>(
    state: &mut MatchState,
    rng: &mut R,
) -> MatchRunnerResult<MatchRunResult> {
    run_match_with_limit(state, rng, DEFAULT_MAX_ITERATIONS)
}

pub fn run_match_with_limit<R: Rng + ?Sized>(
    state: &mut MatchState,
    rng: &mut R,
    max_iterations: usize,
) -> MatchRunnerResult<MatchRunResult> {
    let registry = AggregatorRegistry::with_default_aggregators();
    run_match_with_registry(state, registry, rng, max_iterations)
}

pub fn run_match_with_registry<R: Rng + ?Sized>(
    state: &mut MatchState,
    mut aggregators: AggregatorRegistry,
    rng: &mut R,
    max_iterations: usize,
) -> MatchRunnerResult<MatchRunResult> {
    let mut raw_sink = InMemorySink::new();
    run_loop(state, &mut raw_sink, &mut aggregators, rng, max_iterations)?;
    Ok(MatchRunResult::new(raw_sink, aggregators))
}

pub fn run_loop<R: Rng + ?Sized>(
    state: &mut MatchState,
    raw_sink: &mut InMemorySink,
    aggregators: &mut AggregatorRegistry,
    _rng: &mut R,
    max_iterations: usize,
) -> MatchRunnerResult<usize> {
    let mut iterations = 0;
    let mut dual_sink = DualEventSink::new(raw_sink, aggregators);

    while !state.is_match_finished() {
        if iterations >= max_iterations {
            return Err(MatchRunnerError::MaxIterationsExceeded { max_iterations });
        }
        step_call_to_action(state, &mut dual_sink)?;
        iterations += 1;
    }

    Ok(iterations)
}