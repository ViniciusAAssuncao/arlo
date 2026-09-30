use crate::action::DuelKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FoulOrigin {
    Lineup,
    ContactDuel(DuelKind),
    LineFault,
    CallToAction,
    Drive,
    ShotAttempt,
    OutOfBounds,
}
