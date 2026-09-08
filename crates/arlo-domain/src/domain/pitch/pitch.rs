use crate::domain::pitch::coordinates::PitchCoordinates;
use crate::domain::pitch::zone::PitchZone;
use crate::domain::sport_constants::{
    AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM, DEFAULT_ARTRO_LATERAL_OFFSET_MIRIM,
    FIRST_ZONE_DEPTH_MIRIM, PITCH_LENGTH_MIRIM_MAX, PITCH_LENGTH_MIRIM_MIN, PITCH_WIDTH_MIRIM_MAX,
    PITCH_WIDTH_MIRIM_MIN,
};
use crate::domain::validation::validate_float_range;
use crate::error::DomainResult;
use arlo_math::units::{Length, Position, MIRIM_TO_METERS};
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

    pub fn zone_at_mirim(&self, x_mirim: f64, y_mirim: f64) -> PitchZone {
        let length = self.length_mirim();
        let width = self.width_mirim();
        let dist_to_goal = x_mirim.min(length - x_mirim).max(0.0);
        if dist_to_goal <= FIRST_ZONE_DEPTH_MIRIM {
            PitchZone::FirstZone
        } else if dist_to_goal <= FIRST_ZONE_DEPTH_MIRIM + AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM {
            PitchZone::SecondZone
        } else {
            let center_y = width / 2.0;
            let lateral_dist = (y_mirim - center_y).abs();
            if lateral_dist >= DEFAULT_ARTRO_LATERAL_OFFSET_MIRIM - 5.0 {
                PitchZone::Corridor
            } else {
                PitchZone::Central
            }
        }
    }

    pub fn zone_at_meters(&self, x_m: f64, y_m: f64) -> PitchZone {
        self.zone_at_mirim(x_m / MIRIM_TO_METERS, y_m / MIRIM_TO_METERS)
    }

    pub fn zone_at_position(&self, pos: Position) -> PitchZone {
        self.zone_at_meters(pos.raw().0, pos.raw().1)
    }

    pub fn zone_at_coordinates(&self, coords: PitchCoordinates) -> PitchZone {
        self.zone_at_meters(coords.x_meters(), coords.y_meters())
    }
}
