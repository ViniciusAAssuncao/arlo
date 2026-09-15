use crate::match_decision::play_outcome::DetailedPlayOutcome;
use arlo_manager_control::RequiredManagerDecision;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayStepOutcome {
    Resolved(DetailedPlayOutcome),
    Pending(Vec<RequiredManagerDecision>),
}

impl PlayStepOutcome {
    pub fn is_resolved(&self) -> bool {
        matches!(self, Self::Resolved(_))
    }

    pub fn is_pending(&self) -> bool {
        matches!(self, Self::Pending(_))
    }
}