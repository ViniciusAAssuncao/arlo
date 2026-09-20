use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClubHistoryEntry {
    id: Uuid,
    player_id: Uuid,
    team_id: Uuid,
    joined_year: i64,
    left_year: Option<i64>,
    created_at_unix_seconds: i64,
}

impl ClubHistoryEntry {
    pub fn new(
        id: Uuid,
        player_id: Uuid,
        team_id: Uuid,
        joined_year: i64,
        left_year: Option<i64>,
        created_at_unix_seconds: i64,
    ) -> Self {
        Self {
            id,
            player_id,
            team_id,
            joined_year,
            left_year,
            created_at_unix_seconds,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn joined_year(&self) -> i64 {
        self.joined_year
    }

    pub fn left_year(&self) -> Option<i64> {
        self.left_year
    }

    pub fn created_at_unix_seconds(&self) -> i64 {
        self.created_at_unix_seconds
    }
}
