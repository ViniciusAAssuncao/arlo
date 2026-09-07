use arlo_domain::{Position, SlotRole};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SlotAssignment {
    formation_slot_index: usize,
    position: Position,
    player_id: Uuid,
    slot_role: SlotRole,
}

impl SlotAssignment {
    pub fn new(
        formation_slot_index: usize,
        position: Position,
        player_id: Uuid,
        slot_role: SlotRole,
    ) -> Self {
        Self {
            formation_slot_index,
            position,
            player_id,
            slot_role,
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
}
