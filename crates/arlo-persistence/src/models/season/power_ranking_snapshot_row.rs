use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct PowerRankingSnapshotRow {
    pub id: String,
    pub season_instance_id: String,
    pub year: i64,
    pub day_of_year: i64,
    pub model_version: i64,
}
