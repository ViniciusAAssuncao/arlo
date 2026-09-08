use crate::instructions::player::builder::PlayerInstructionsBuilder;
use crate::instructions::player::in_possession::InPossessionPlayerInstructions;
use crate::instructions::player::out_of_possession::OutOfPossessionPlayerInstructions;
use crate::instructions::player::transition::TransitionPlayerInstructions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct PlayerInstructions {
    in_possession: InPossessionPlayerInstructions,
    out_of_possession: OutOfPossessionPlayerInstructions,
    transition: TransitionPlayerInstructions,
}

impl PlayerInstructions {
    pub fn new(
        in_possession: InPossessionPlayerInstructions,
        out_of_possession: OutOfPossessionPlayerInstructions,
        transition: TransitionPlayerInstructions,
    ) -> Self {
        Self {
            in_possession,
            out_of_possession,
            transition,
        }
    }

    pub fn builder() -> PlayerInstructionsBuilder {
        PlayerInstructionsBuilder::default()
    }

    pub fn in_possession(&self) -> &InPossessionPlayerInstructions {
        &self.in_possession
    }

    pub fn out_of_possession(&self) -> &OutOfPossessionPlayerInstructions {
        &self.out_of_possession
    }

    pub fn transition(&self) -> &TransitionPlayerInstructions {
        &self.transition
    }
}