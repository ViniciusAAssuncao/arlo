use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PunishmentKind {
    LossOfDown,
    InvalidatePreviousPlay,
    YardageLoss,
    LossOfDrive,
    TimePenalty,
    Expulsion,
}