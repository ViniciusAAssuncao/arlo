use crate::domain::position::Position;
use crate::domain::validation::validate_integer_range;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerPosition {
    position: Position,
    proficiency: i32,
}

impl PlayerPosition {
    pub fn new(position: Position, proficiency: i32) -> DomainResult<Self> {
        validate_integer_range(proficiency, 0, 10, "proficiency")?;
        Ok(Self {
            position,
            proficiency,
        })
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn proficiency(&self) -> i32 {
        self.proficiency
    }
}