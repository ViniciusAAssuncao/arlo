use crate::instructions::player::PlayerInstructions;
use arlo_domain::{Position, SlotRole};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SlotAssignment {
    formation_slot_index: usize,
    position: Position,
    player_id: Uuid,
    slot_role: SlotRole,
    player_instructions: PlayerInstructions,
}

impl SlotAssignment {
    pub fn new(
        formation_slot_index: usize,
        position: Position,
        player_id: Uuid,
        slot_role: SlotRole,
        player_instructions: PlayerInstructions,
    ) -> Self {
        Self {
            formation_slot_index,
            position,
            player_id,
            slot_role,
            player_instructions,
        }
    }

    pub fn formation_slot_index(&self) -> usize {
        self.formation_slot_index
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn slot_role(&self) -> SlotRole {
        self.slot_role
    }

    pub fn player_instructions(&self) -> &PlayerInstructions {
        &self.player_instructions
    }
}
