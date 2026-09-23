use arlo_engine::world_state::{step_call_to_action, MatchState, PlayStepOutcome};
use arlo_engine::EngineResult;
use arlo_events::InMemorySink;
use arlo_manager_control::ManagerDecisionInbox;
use arlo_stats::AggregatorRegistry;

pub struct MatchSession {
    state: MatchState,
    registry: AggregatorRegistry,
    sink: InMemorySink,
    inbox: ManagerDecisionInbox,
}

impl MatchSession {
    pub fn new(state: MatchState, registry: AggregatorRegistry) -> Self {
        Self {
            state,
            registry,
            sink: InMemorySink::new(),
            inbox: ManagerDecisionInbox::new(),
        }
    }

    pub fn with_sink(state: MatchState, registry: AggregatorRegistry, sink: InMemorySink) -> Self {
        Self {
            state,
            registry,
            sink,
            inbox: ManagerDecisionInbox::new(),
        }
    }

    pub fn with_inbox(
        state: MatchState,
        registry: AggregatorRegistry,
        sink: InMemorySink,
        inbox: ManagerDecisionInbox,
    ) -> Self {
        Self {
            state,
            registry,
            sink,
            inbox,
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

    pub fn inbox(&self) -> &ManagerDecisionInbox {
        &self.inbox
    }

    pub fn is_finished(&self) -> bool {
        self.state.is_match_finished()
    }

    pub fn step(&mut self) -> EngineResult<PlayStepOutcome> {
        let prev_count = self.sink.len();
        let outcome = step_call_to_action(&mut self.state, &self.inbox, &mut self.sink)?;
        for envelope in &self.sink.events()[prev_count..] {
            self.registry.handle_envelope(envelope);
        }
        Ok(outcome)
    }

    pub fn step_until_finished(&mut self) -> EngineResult<Vec<PlayStepOutcome>> {
        let mut outcomes = Vec::new();
        while !self.is_finished() {
            let outcome = self.step()?;
            let is_pending = outcome.is_pending();
            outcomes.push(outcome);
            if is_pending {
                break;
            }
        }
        Ok(outcomes)
    }
}
