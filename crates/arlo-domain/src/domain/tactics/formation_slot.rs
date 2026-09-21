use crate::domain::position::{Position, PositionLine};
use crate::domain::tactics::slot_role::SlotRole;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FormationSlot {
    offensive_position: Position,
    defensive_position: Position,
    role: SlotRole,
    pitch_length_ratio: Option<f64>,
    pitch_width_ratio: Option<f64>,
}

impl FormationSlot {
    pub fn new(
        position: Position,
        role: SlotRole,
        pitch_length_ratio: Option<f64>,
        pitch_width_ratio: Option<f64>,
    ) -> Self {
        Self {
            offensive_position: position,
            defensive_position: position,
            role,
            pitch_length_ratio,
            pitch_width_ratio,
        }
    }

    pub fn without_coordinates(position: Position, role: SlotRole) -> Self {
        Self::new(position, role, None, None)
    }

    pub fn with_coordinates(
        position: Position,
        role: SlotRole,
        pitch_length_ratio: Option<f64>,
        pitch_width_ratio: Option<f64>,
    ) -> Self {
        Self::new(position, role, pitch_length_ratio, pitch_width_ratio)
    }

    pub fn with_dual_positions(
        offensive_position: Position,
        defensive_position: Position,
        role: SlotRole,
        pitch_length_ratio: Option<f64>,
        pitch_width_ratio: Option<f64>,
    ) -> Self {
        Self {
            offensive_position,
            defensive_position,
            role,
            pitch_length_ratio,
            pitch_width_ratio,
        }
    }

    pub fn with_dual_positions_without_coordinates(
        offensive_position: Position,
        defensive_position: Position,
        role: SlotRole,
    ) -> Self {
        Self::with_dual_positions(
            offensive_position,
            defensive_position,
            role,
            None,
            None,
        )
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

    pub fn pitch_length_ratio(&self) -> Option<f64> {
        self.pitch_length_ratio
    }

    pub fn pitch_width_ratio(&self) -> Option<f64> {
        self.pitch_width_ratio
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