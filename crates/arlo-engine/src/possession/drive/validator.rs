use crate::possession::drive::artrine_identity::{ArtrineCarrier, TrueArtrine};
use crate::possession::drive::geometry::segment_intersects_artro;
use arlo_domain::pitch::Artro;
use arlo_math::units::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DriveValidationStatus {
    Valid,
    NotIntersected,
    AerialReceptionExcluded,
    InvalidIdentity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DriveValidationResult {
    status: DriveValidationStatus,
}

impl DriveValidationResult {
    pub fn new(status: DriveValidationStatus) -> Self {
        Self { status }
    }

    pub fn status(&self) -> DriveValidationStatus {
        self.status
    }

    pub fn is_valid(&self) -> bool {
        self.status == DriveValidationStatus::Valid
    }
}

pub fn validate_drive(
    _artrine: TrueArtrine,
    start_pos: Position,
    end_pos: Position,
    artro: &Artro,
    is_aerial_reception: bool,
) -> DriveValidationResult {
    if is_aerial_reception {
        return DriveValidationResult::new(DriveValidationStatus::AerialReceptionExcluded);
    }

    if !segment_intersects_artro(start_pos, end_pos, artro) {
        return DriveValidationResult::new(DriveValidationStatus::NotIntersected);
    }

    DriveValidationResult::new(DriveValidationStatus::Valid)
}

pub fn validate_continuous_trajectory(
    _artrine: TrueArtrine,
    segments: &[(Position, Position)],
    artro: &Artro,
    is_aerial_reception: bool,
) -> DriveValidationResult {
    if is_aerial_reception {
        return DriveValidationResult::new(DriveValidationStatus::AerialReceptionExcluded);
    }

    for &(start_pos, end_pos) in segments {
        if segment_intersects_artro(start_pos, end_pos, artro) {
            return DriveValidationResult::new(DriveValidationStatus::Valid);
        }
    }

    DriveValidationResult::new(DriveValidationStatus::NotIntersected)
}

pub fn validate_carrier_drive(
    carrier: ArtrineCarrier,
    start_pos: Position,
    end_pos: Position,
    artro: &Artro,
    is_aerial_reception: bool,
) -> DriveValidationResult {
    match carrier {
        ArtrineCarrier::True(true_artrine) => {
            validate_drive(true_artrine, start_pos, end_pos, artro, is_aerial_reception)
        }
        ArtrineCarrier::False(_) => {
            DriveValidationResult::new(DriveValidationStatus::InvalidIdentity)
        }
    }
}

pub fn validate_carrier_continuous_trajectory(
    carrier: ArtrineCarrier,
    segments: &[(Position, Position)],
    artro: &Artro,
    is_aerial_reception: bool,
) -> DriveValidationResult {
    match carrier {
        ArtrineCarrier::True(true_artrine) => {
            validate_continuous_trajectory(true_artrine, segments, artro, is_aerial_reception)
        }
        ArtrineCarrier::False(_) => {
            DriveValidationResult::new(DriveValidationStatus::InvalidIdentity)
        }
    }
}

pub fn is_drive_valid(
    artrine: TrueArtrine,
    start_pos: Position,
    end_pos: Position,
    artro: &Artro,
    is_aerial_reception: bool,
) -> bool {
    validate_drive(artrine, start_pos, end_pos, artro, is_aerial_reception).is_valid()
}

pub fn is_trajectory_drive_valid(
    artrine: TrueArtrine,
    segments: &[(Position, Position)],
    artro: &Artro,
    is_aerial_reception: bool,
) -> bool {
    validate_continuous_trajectory(artrine, segments, artro, is_aerial_reception).is_valid()
}