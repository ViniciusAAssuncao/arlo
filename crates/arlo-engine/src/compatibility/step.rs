use crate::compatibility::match_state::MatchState;
use crate::error::{EngineError, EngineResult};
use arlo_events::EventSink;
use arlo_manager_control::{ManagerDecisionInbox, RequiredManagerDecision};

#[derive(Debug, Clone, PartialEq)]
pub enum PlayStepOutcome {
    Resolved(u64),
    Pending(Vec<RequiredManagerDecision>),
}

impl PlayStepOutcome {
    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending(_))
    }

    pub fn is_resolved(&self) -> bool {
        matches!(self, Self::Resolved(_))
    }
}

pub fn step_call_to_action(
    state: &mut MatchState,
    _inbox: &ManagerDecisionInbox,
    _sink: &mut impl EventSink,
) -> EngineResult<PlayStepOutcome> {
    if state.is_match_finished() {
        return Err(EngineError::SimulationCompleted);
    }

    Err(EngineError::NotImplemented(
        "Simulation step engine is under active construction",
    ))
}