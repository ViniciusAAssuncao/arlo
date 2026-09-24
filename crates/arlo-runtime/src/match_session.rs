use arlo_engine::{MatchInput, MatchPhase, MatchState, StepOutcome};
use arlo_events::{EventSink, InMemorySink};
use arlo_manager_control::ManagerDecisionInbox;
use arlo_match_runner::{resolve_segment, MatchRunnerResult};
use arlo_stats::AggregatorRegistry;
use arlo_tactics::PlayCall;

pub struct MatchSession {
    input: MatchInput,
    state: MatchState,
    registry: AggregatorRegistry,
    sink: InMemorySink,
    inbox: ManagerDecisionInbox,
    play_calls: Vec<PlayCall>,
}

impl MatchSession {
    pub fn new(input: MatchInput, state: MatchState, registry: AggregatorRegistry) -> Self {
        Self {
            input,
            state,
            registry,
            sink: InMemorySink::new(),
            inbox: ManagerDecisionInbox::new(),
            play_calls: Vec::new(),
        }
    }

    pub fn with_sink(
        input: MatchInput,
        state: MatchState,
        registry: AggregatorRegistry,
        sink: InMemorySink,
    ) -> Self {
        Self {
            input,
            state,
            registry,
            sink,
            inbox: ManagerDecisionInbox::new(),
            play_calls: Vec::new(),
        }
    }

    pub fn with_inbox(
        input: MatchInput,
        state: MatchState,
        registry: AggregatorRegistry,
        sink: InMemorySink,
        inbox: ManagerDecisionInbox,
        play_calls: Vec<PlayCall>,
    ) -> Self {
        Self {
            input,
            state,
            registry,
            sink,
            inbox,
            play_calls,
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
        self.state.phase() == MatchPhase::Finished
    }

    pub fn step(&mut self) -> MatchRunnerResult<StepOutcome> {
        let result = resolve_segment(&self.input, &mut self.state, &self.inbox, &self.play_calls)?;
        let (events, outcome) = result.into_parts();
        for envelope in &events {
            self.registry.handle_envelope(envelope);
        }
        self.sink.record_all(events);
        Ok(outcome)
    }

    pub fn step_until_finished(&mut self) -> MatchRunnerResult<Vec<StepOutcome>> {
        let mut outcomes = Vec::new();
        while !self.is_finished() {
            let outcome = self.step()?;
            let is_pending = matches!(outcome, StepOutcome::AwaitingDecision(_));
            outcomes.push(outcome);
            if is_pending {
                break;
            }
        }
        Ok(outcomes)
    }
}
