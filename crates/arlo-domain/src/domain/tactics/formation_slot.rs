use crate::domain::position::{Position, PositionLine};
use crate::domain::sport_constants::{NORMALIZED_RATIO_MAX, NORMALIZED_RATIO_MIN};
use crate::domain::validation::validate_float_range;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FormationSlot {
    position: Position,
    pitch_length_ratio: f64,
    pitch_width_ratio: f64,
}

impl FormationSlot {
    pub fn new(
        position: Position,
        pitch_length_ratio: f64,
        pitch_width_ratio: f64,
    ) -> DomainResult<Self> {
        validate_float_range(
            pitch_length_ratio,
            NORMALIZED_RATIO_MIN,
            NORMALIZED_RATIO_MAX,
            "pitch_length_ratio",
        )?;
        validate_float_range(
            pitch_width_ratio,
            NORMALIZED_RATIO_MIN,
            NORMALIZED_RATIO_MAX,
            "pitch_width_ratio",
        )?;

        Ok(Self {
            position,
            pitch_length_ratio,
            pitch_width_ratio,
        })
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn pitch_length_ratio(&self) -> f64 {
        self.pitch_length_ratio
    }

    pub fn pitch_width_ratio(&self) -> f64 {
        self.pitch_width_ratio
    }

    pub fn line(&self) -> PositionLine {
        self.position.line()
    }

    pub fn position_line(&self) -> PositionLine {
        self.position.line()
    }
}
