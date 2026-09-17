use crate::possession::drive::artrine_identity::{ArtrineCarrier, TrueArtrine};
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

    let min_x = start_pos.raw().0.min(end_pos.raw().0);
    let max_x = start_pos.raw().0.max(end_pos.raw().0);
    let artro_x = artro.x().value();
    let half_size = artro.size().value() / 2.0;

    if artro_x + half_size < min_x || artro_x - half_size > max_x {
        return DriveValidationResult::new(DriveValidationStatus::NotIntersected);
    }

    DriveValidationResult::new(DriveValidationStatus::Valid)
}

pub fn validate_continuous_trajectory(
    artrine: TrueArtrine,
    segments: &[(Position, Position)],
    artro: &Artro,
    is_aerial_reception: bool,
) -> DriveValidationResult {
    if is_aerial_reception {
        return DriveValidationResult::new(DriveValidationStatus::AerialReceptionExcluded);
    }

    for &(start_pos, end_pos) in segments {
        let res = validate_drive(artrine, start_pos, end_pos, artro, false);
        if res.is_valid() {
            return res;
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
