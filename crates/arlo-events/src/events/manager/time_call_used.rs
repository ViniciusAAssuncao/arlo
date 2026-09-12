use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimeCallReason {
    Standard,
    KickFoulRealignment,
}

impl TimeCallReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Standard => "Standard",
            Self::KickFoulRealignment => "KickFoulRealignment",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeCallUsed {
    team_id: Uuid,
    remaining_time_calls_after: u32,
    reason: TimeCallReason,
}

impl TimeCallUsed {
    pub fn new(team_id: Uuid, remaining_time_calls_after: u32, reason: TimeCallReason) -> Self {
        Self {
            team_id,
            remaining_time_calls_after,
            reason,
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn remaining_time_calls_after(&self) -> u32 {
        self.remaining_time_calls_after
    }

    pub fn reason(&self) -> TimeCallReason {
        self.reason
    }
}