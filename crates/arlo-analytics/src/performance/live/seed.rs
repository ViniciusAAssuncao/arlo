use arlo_domain::{Position, SlotRole};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitialParticipantSeed {
    player_id: Uuid,
    team_id: Uuid,
    offensive_position: Position,
    defensive_position: Position,
    slot_role: SlotRole,
    is_active: bool,
}

impl InitialParticipantSeed {
    pub fn new(
        player_id: Uuid,
        team_id: Uuid,
        offensive_position: Position,
        defensive_position: Position,
        slot_role: SlotRole,
        is_active: bool,
    ) -> Self {
        Self {
            player_id,
            team_id,
            offensive_position,
            defensive_position,
            slot_role,
            is_active,
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn offensive_position(&self) -> Position {
        self.offensive_position
    }

    pub fn defensive_position(&self) -> Position {
        self.defensive_position
    }

    pub fn slot_role(&self) -> SlotRole {
        self.slot_role
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}
