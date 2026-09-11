use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AvailabilityStatus {
    Active,
    Suspended,
    Expelled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerAvailabilityChanged {
    player_id: Uuid,
    team_id: Uuid,
    previous_status: AvailabilityStatus,
    new_status: AvailabilityStatus,
    remaining_seconds: Option<f64>,
}

impl PlayerAvailabilityChanged {
    pub fn new(
        player_id: Uuid,
        team_id: Uuid,
        previous_status: AvailabilityStatus,
        new_status: AvailabilityStatus,
        remaining_seconds: Option<f64>,
    ) -> Self {
        Self {
            player_id,
            team_id,
            previous_status,
            new_status,
            remaining_seconds,
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn previous_status(&self) -> AvailabilityStatus {
        self.previous_status
    }

    pub fn new_status(&self) -> AvailabilityStatus {
        self.new_status
    }

    pub fn remaining_seconds(&self) -> Option<f64> {
        self.remaining_seconds
    }
}