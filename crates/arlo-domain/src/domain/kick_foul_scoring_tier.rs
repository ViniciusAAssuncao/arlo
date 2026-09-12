use crate::domain::pitch::zone::PitchZone;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KickFoulScoringTier {
    Standard,
    FirstZone,
}

impl KickFoulScoringTier {
    pub fn from_zone(zone: PitchZone) -> Self {
        match zone {
            PitchZone::FirstZone => Self::FirstZone,
            _ => Self::Standard,
        }
    }
}