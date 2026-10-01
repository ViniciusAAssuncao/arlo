use arlo_events::MatchEventEnvelope;
use arlo_manager_control::RequiredManagerDecision;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepOutcome {
    Resolved,
    AwaitingDecision(Vec<RequiredManagerDecision>),
    Finished,
}

#[derive(Debug, Clone)]
pub struct StepResult {
    events: Vec<MatchEventEnvelope>,
    outcome: StepOutcome,
}

impl StepResult {
    pub fn resolved(events: Vec<MatchEventEnvelope>) -> Self {
        Self {
            events,
            outcome: StepOutcome::Resolved,
        }
    }

    pub fn awaiting_decision(decisions: Vec<RequiredManagerDecision>) -> Self {
        Self {
            events: Vec::new(),
            outcome: StepOutcome::AwaitingDecision(decisions),
        }
    }

    pub fn finished(events: Vec<MatchEventEnvelope>) -> Self {
        Self {
            events,
            outcome: StepOutcome::Finished,
        }
    }

    pub fn events(&self) -> &[MatchEventEnvelope] {
        &self.events
    }
    pub fn outcome(&self) -> &StepOutcome {
        &self.outcome
    }
    pub fn into_parts(self) -> (Vec<MatchEventEnvelope>, StepOutcome) {
        (self.events, self.outcome)
    }
}
