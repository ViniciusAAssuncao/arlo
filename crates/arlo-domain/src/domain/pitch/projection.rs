use crate::domain::pitch::coordinates::PitchCoordinates;
use crate::domain::pitch::pitch::Pitch;
use crate::domain::tactics::{Formation, FormationSlot};
use arlo_math::units::Length;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProjectionDirection {
    Standard,
    Mirrored,
}

pub fn project_ratio(pitch: &Pitch, rx: f64, ry: f64) -> PitchCoordinates {
    let x = Length::new(rx * pitch.length().value());
    let y = Length::new(ry * pitch.width().value());
    PitchCoordinates::new(x, y)
}

pub fn project_ratio_mirrored(pitch: &Pitch, rx: f64, ry: f64) -> PitchCoordinates {
    let x = Length::new((1.0 - rx) * pitch.length().value());
    let y = Length::new((1.0 - ry) * pitch.width().value());
    PitchCoordinates::new(x, y)
}

pub fn project_slot(pitch: &Pitch, slot: &FormationSlot) -> PitchCoordinates {
    project_ratio(pitch, slot.pitch_length_ratio(), slot.pitch_width_ratio())
}

pub fn project_slot_mirrored(pitch: &Pitch, slot: &FormationSlot) -> PitchCoordinates {
    project_ratio_mirrored(pitch, slot.pitch_length_ratio(), slot.pitch_width_ratio())
}

pub fn project_slot_with_direction(
    pitch: &Pitch,
    slot: &FormationSlot,
    direction: ProjectionDirection,
) -> PitchCoordinates {
    match direction {
        ProjectionDirection::Standard => project_slot(pitch, slot),
        ProjectionDirection::Mirrored => project_slot_mirrored(pitch, slot),
    }
}

pub fn project_formation(pitch: &Pitch, formation: &Formation) -> Vec<PitchCoordinates> {
    formation
        .slots()
        .iter()
        .map(|slot| project_slot(pitch, slot))
        .collect()
}

pub fn project_formation_mirrored(
    pitch: &Pitch,
    formation: &Formation,
) -> Vec<PitchCoordinates> {
    formation
        .slots()
        .iter()
        .map(|slot| project_slot_mirrored(pitch, slot))
        .collect()
}

pub fn project_formation_with_direction(
    pitch: &Pitch,
    formation: &Formation,
    direction: ProjectionDirection,
) -> Vec<PitchCoordinates> {
    match direction {
        ProjectionDirection::Standard => project_formation(pitch, formation),
        ProjectionDirection::Mirrored => project_formation_mirrored(pitch, formation),
    }
}
