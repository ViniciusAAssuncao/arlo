use arlo_manager_control::RequiredManagerDecision;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationStatus {
    Ready,
    InProgress,
    PausedForDecision(RequiredManagerDecision),
    Completed,
    Aborted(String),
}

impl SimulationStatus {
    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Ready)
    }

    pub fn is_in_progress(&self) -> bool {
        matches!(self, Self::InProgress)
    }

    pub fn is_paused(&self) -> bool {
        matches!(self, Self::PausedForDecision(_))
    }

    pub fn is_completed(&self) -> bool {
        matches!(self, Self::Completed)
    }

    pub fn is_aborted(&self) -> bool {
        matches!(self, Self::Aborted(_))
    }

    pub fn pending_decision(&self) -> Option<&RequiredManagerDecision> {
        match self {
            Self::PausedForDecision(d) => Some(d),
            _ => None,
        }
    }
}