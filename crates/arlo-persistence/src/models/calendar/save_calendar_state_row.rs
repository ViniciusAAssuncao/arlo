use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct SaveCalendarStateRow {
    pub save_uuid: String,
    pub calendar_system_id: String,
    pub current_year: i64,
    pub current_day_of_year: i32,
    pub assigned_at_unix_seconds: i64,
}

impl SaveCalendarStateRow {
    pub fn new(
        save_uuid: Uuid,
        calendar_system_id: Uuid,
        current_year: i64,
        current_day_of_year: u32,
        assigned_at_unix_seconds: i64,
    ) -> Self {
        Self {
            save_uuid: save_uuid.to_string(),
            calendar_system_id: calendar_system_id.to_string(),
            current_year,
            current_day_of_year: current_day_of_year as i32,
            assigned_at_unix_seconds,
        }
    }
}
