use arlo_engine::match_decision::DetailedPlayOutcome;
use arlo_engine::world_state::{step_call_to_action, MatchState};
use arlo_engine::EngineResult;
use arlo_events::InMemorySink;
use arlo_stats::AggregatorRegistry;

pub struct MatchSession {
    state: MatchState,
    registry: AggregatorRegistry,
    sink: InMemorySink,
}

impl MatchSession {
    pub fn new(state: MatchState, registry: AggregatorRegistry) -> Self {
        Self {
            state,
            registry,
            sink: InMemorySink::new(),
        }
    }

    pub fn with_sink(state: MatchState, registry: AggregatorRegistry, sink: InMemorySink) -> Self {
        Self {
            state,
            registry,
            sink,
        }
    }

    pub fn state(&self) -> &MatchState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut MatchState {
        &mut self.state
    }

    pub fn registry(&self) -> &AggregatorRegistry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut AggregatorRegistry {
        &mut self.registry
    }

    pub fn sink(&self) -> &InMemorySink {
        &self.sink
    }

    pub fn sink_mut(&mut self) -> &mut InMemorySink {
        &mut self.sink
    }

    pub fn is_finished(&self) -> bool {
        self.state.is_match_finished()
    }

    pub fn step(&mut self) -> EngineResult<DetailedPlayOutcome> {
        let prev_count = self.sink.len();
        let outcome = step_call_to_action(&mut self.state, &mut self.sink)?;
        for envelope in &self.sink.events()[prev_count..] {
            self.registry.handle_envelope(envelope);
        }
        Ok(outcome)
    }

    pub fn step_until_finished(&mut self) -> EngineResult<Vec<DetailedPlayOutcome>> {
        let mut outcomes = Vec::new();
        while !self.is_finished() {
            outcomes.push(self.step()?);
        }
        Ok(outcomes)
    }
}
