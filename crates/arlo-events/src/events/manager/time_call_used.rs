use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimeCallUsed {
    team_id: Uuid,
    remaining_time_calls_after: u32,
}

impl TimeCallUsed {
    pub fn new(team_id: Uuid, remaining_time_calls_after: u32) -> Self {
        Self {
            team_id,
            remaining_time_calls_after,
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn remaining_time_calls_after(&self) -> u32 {
        self.remaining_time_calls_after
    }
}
