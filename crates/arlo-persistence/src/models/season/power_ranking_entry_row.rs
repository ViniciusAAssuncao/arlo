use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct PowerRankingEntryRow {
    pub snapshot_id: String,
    pub team_id: String,
    pub rank: i64,
    pub rating: f64,
    pub initial_rating: f64,
    pub games_rated: i64,
}
