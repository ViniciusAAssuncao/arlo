use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LeagueMovementRule {
    None,
    Automatic { count: u32 },
    PlayoffStage { stage_order_index: u32, count: u32 },
}

impl LeagueMovementRule {
    pub fn count(&self) -> Option<u32> {
        match self {
            Self::None => None,
            Self::Automatic { count } => Some(*count),
            Self::PlayoffStage { count, .. } => Some(*count),
        }
    }

    pub fn stage_order_index(&self) -> Option<u32> {
        match self {
            Self::PlayoffStage { stage_order_index, .. } => Some(*stage_order_index),
            _ => None,
        }
    }
}
