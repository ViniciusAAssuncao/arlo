use crate::domain::sport_constants::{
    PITCH_LENGTH_MIRIM_MAX, PITCH_LENGTH_MIRIM_MIN, PITCH_WIDTH_MIRIM_MAX, PITCH_WIDTH_MIRIM_MIN,
};
use crate::domain::validation::validate_float_range;
use crate::error::DomainResult;
use arlo_math::units::{Length, MIRIM_TO_METERS};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Pitch {
    length: Length,
    width: Length,
}

impl Pitch {
    pub fn from_mirim(length_mirim: f64, width_mirim: f64) -> DomainResult<Self> {
        validate_float_range(
            length_mirim,
            PITCH_LENGTH_MIRIM_MIN,
            PITCH_LENGTH_MIRIM_MAX,
            "pitch_length_mirim",
        )?;
        validate_float_range(
            width_mirim,
            PITCH_WIDTH_MIRIM_MIN,
            PITCH_WIDTH_MIRIM_MAX,
            "pitch_width_mirim",
        )?;

        let length = Length::new(length_mirim * MIRIM_TO_METERS);
        let width = Length::new(width_mirim * MIRIM_TO_METERS);

        Ok(Self { length, width })
    }

    pub fn length(&self) -> Length {
        self.length
    }

    pub fn width(&self) -> Length {
        self.width
    }

    pub fn length_mirim(&self) -> f64 {
        self.length.value() / MIRIM_TO_METERS
    }

    pub fn width_mirim(&self) -> f64 {
        self.width.value() / MIRIM_TO_METERS
    }
}
