use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct SeasonInstanceRow {
    pub id: String,
    pub competition_id: String,
    pub reference_year: i64,
    pub current_stage_order_index: i32,
    pub status: String,
    pub created_at_unix_seconds: i64,
}

impl SeasonInstanceRow {
    pub fn new(
        id: Uuid,
        competition_id: Uuid,
        reference_year: i64,
        current_stage_order_index: u32,
        status: impl Into<String>,
        created_at_unix_seconds: i64,
    ) -> Self {
        Self {
            id: id.to_string(),
            competition_id: competition_id.to_string(),
            reference_year,
            current_stage_order_index: current_stage_order_index as i32,
            status: status.into(),
            created_at_unix_seconds,
        }
    }
}
