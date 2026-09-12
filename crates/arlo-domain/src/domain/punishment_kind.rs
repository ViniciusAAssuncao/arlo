use crate::domain::sport_constants::{
    PUNISHMENT_SEVERITY_EXPULSION, PUNISHMENT_SEVERITY_INVALIDATE_PREVIOUS_PLAY,
    PUNISHMENT_SEVERITY_KICK_FOUL_AWARDED, PUNISHMENT_SEVERITY_LOSS_OF_DOWN,
    PUNISHMENT_SEVERITY_LOSS_OF_DRIVE, PUNISHMENT_SEVERITY_TIME_PENALTY,
    PUNISHMENT_SEVERITY_YARDAGE_LOSS,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PunishmentKind {
    LossOfDown,
    InvalidatePreviousPlay,
    YardageLoss,
    LossOfDrive,
    KickFoulAwarded,
    TimePenalty,
    Expulsion,
}

impl PunishmentKind {
    pub fn relative_severity(&self) -> f64 {
        match self {
            Self::YardageLoss => PUNISHMENT_SEVERITY_YARDAGE_LOSS,
            Self::LossOfDown => PUNISHMENT_SEVERITY_LOSS_OF_DOWN,
            Self::LossOfDrive => PUNISHMENT_SEVERITY_LOSS_OF_DRIVE,
            Self::KickFoulAwarded => PUNISHMENT_SEVERITY_KICK_FOUL_AWARDED,
            Self::TimePenalty => PUNISHMENT_SEVERITY_TIME_PENALTY,
            Self::Expulsion => PUNISHMENT_SEVERITY_EXPULSION,
            Self::InvalidatePreviousPlay => PUNISHMENT_SEVERITY_INVALIDATE_PREVIOUS_PLAY,
        }
    }
}