use crate::domain::position::{Position, PositionLine};
use crate::domain::tactics::slot_role::SlotRole;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FormationSlot {
    offensive_position: Position,
    defensive_position: Position,
    role: SlotRole,
}

impl FormationSlot {
    pub fn new(position: Position, role: SlotRole) -> Self {
        Self {
            offensive_position: position,
            defensive_position: position,
            role,
        }
    }

    pub fn with_dual_positions(
        offensive_position: Position,
        defensive_position: Position,
        role: SlotRole,
    ) -> Self {
        Self {
            offensive_position,
            defensive_position,
            role,
        }
    }

    pub fn position(&self) -> Position {
        self.offensive_position
    }

    pub fn offensive_position(&self) -> Position {
        self.offensive_position
    }

    pub fn defensive_position(&self) -> Position {
        self.defensive_position
    }

    pub fn role(&self) -> SlotRole {
        self.role
    }

    pub fn line(&self) -> PositionLine {
        self.offensive_position.line()
    }

    pub fn position_line(&self) -> PositionLine {
        self.offensive_position.line()
    }

    pub fn offensive_line(&self) -> PositionLine {
        self.offensive_position.line()
    }

    pub fn defensive_line(&self) -> PositionLine {
        self.defensive_position.line()
    }
}