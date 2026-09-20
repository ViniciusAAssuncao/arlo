use crate::domain::pitch::zone::PitchZone;
use crate::domain::sport_constants::{
    FIRST_ZONE_DEPTH_MIRIM, PITCH_LENGTH_MIRIM_MAX, PITCH_LENGTH_MIRIM_MIN, PITCH_WIDTH_MIRIM_MAX,
    PITCH_WIDTH_MIRIM_MIN,
};
use crate::domain::validation::validate_float_range;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Pitch {
    length_mirim: f64,
    width_mirim: f64,
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

        Ok(Self {
            length_mirim,
            width_mirim,
        })
    }

    pub fn length_mirim(&self) -> f64 {
        self.length_mirim
    }

    pub fn width_mirim(&self) -> f64 {
        self.width_mirim
    }

    pub fn zone_at_progress(
        &self,
        normalized_progress: f64,
        second_zone_depth_mirim: f64,
    ) -> PitchZone {
        let clamped = normalized_progress.clamp(0.0, 1.0);
        let distance_to_goal = (1.0 - clamped) * self.length_mirim;
        self.zone_at_distance_to_goal(distance_to_goal, second_zone_depth_mirim)
    }

    pub fn zone_at_distance_to_goal(
        &self,
        distance_to_goal_mirim: f64,
        second_zone_depth_mirim: f64,
    ) -> PitchZone {
        if distance_to_goal_mirim <= FIRST_ZONE_DEPTH_MIRIM {
            PitchZone::FirstZone
        } else if distance_to_goal_mirim <= FIRST_ZONE_DEPTH_MIRIM + second_zone_depth_mirim {
            PitchZone::SecondZone
        } else {
            PitchZone::OpenField
        }
    }
}