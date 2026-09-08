use crate::instructions::player::axes::{ReleaseTempo, TransitionUrgency};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct TransitionPlayerInstructions {
    transition_urgency: TransitionUrgency,
    release_tempo: ReleaseTempo,
}

impl TransitionPlayerInstructions {
    pub fn new(
        transition_urgency: TransitionUrgency,
        release_tempo: ReleaseTempo,
    ) -> Self {
        Self {
            transition_urgency,
            release_tempo,
        }
    }

    pub fn transition_urgency(&self) -> TransitionUrgency {
        self.transition_urgency
    }

    pub fn release_tempo(&self) -> ReleaseTempo {
        self.release_tempo
    }
}