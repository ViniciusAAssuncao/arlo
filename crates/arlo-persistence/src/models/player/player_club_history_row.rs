use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct PlayerClubHistoryRow {
    pub id: String,
    pub player_id: String,
    pub team_id: String,
    pub joined_year: i64,
    pub left_year: Option<i64>,
    pub created_at_unix_seconds: i64,
}

impl PlayerClubHistoryRow {
    pub fn new(
        id: Uuid,
        player_id: Uuid,
        team_id: Uuid,
        joined_year: i64,
        left_year: Option<i64>,
        created_at_unix_seconds: i64,
    ) -> Self {
        Self {
            id: id.to_string(),
            player_id: player_id.to_string(),
            team_id: team_id.to_string(),
            joined_year,
            left_year,
            created_at_unix_seconds,
        }
    }
}
