use crate::possession::drive::artrine_identity::{ArtrineCarrier, TrueArtrine};
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
    start_x_mirim: f64,
    end_x_mirim: f64,
    row_x_mirim: f64,
    is_aerial_reception: bool,
) -> DriveValidationResult {
    if is_aerial_reception {
        return DriveValidationResult::new(DriveValidationStatus::AerialReceptionExcluded);
    }

    let min_x = start_x_mirim.min(end_x_mirim);
    let max_x = start_x_mirim.max(end_x_mirim);

    if row_x_mirim >= min_x && row_x_mirim <= max_x {
        DriveValidationResult::new(DriveValidationStatus::Valid)
    } else {
        DriveValidationResult::new(DriveValidationStatus::NotIntersected)
    }
}

pub fn validate_carrier_drive(
    carrier: ArtrineCarrier,
    start_x_mirim: f64,
    end_x_mirim: f64,
    row_x_mirim: f64,
    is_aerial_reception: bool,
) -> DriveValidationResult {
    match carrier {
        ArtrineCarrier::True(true_artrine) => {
            validate_drive(true_artrine, start_x_mirim, end_x_mirim, row_x_mirim, is_aerial_reception)
        }
        ArtrineCarrier::False(_) => {
            DriveValidationResult::new(DriveValidationStatus::InvalidIdentity)
        }
    }
}

pub fn is_drive_valid(
    artrine: TrueArtrine,
    start_x_mirim: f64,
    end_x_mirim: f64,
    row_x_mirim: f64,
    is_aerial_reception: bool,
) -> bool {
    validate_drive(artrine, start_x_mirim, end_x_mirim, row_x_mirim, is_aerial_reception).is_valid()
}

