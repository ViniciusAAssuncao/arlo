use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct PostponementRecordRow {
    pub id: String,
    pub fixture_id: String,
    pub original_year: i64,
    pub original_day_of_year: i32,
    pub new_year: i64,
    pub new_day_of_year: i32,
    pub reason: String,
    pub created_at_unix_seconds: i64,
}

impl PostponementRecordRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        fixture_id: Uuid,
        original_year: i64,
        original_day_of_year: u32,
        new_year: i64,
        new_day_of_year: u32,
        reason: impl Into<String>,
        created_at_unix_seconds: i64,
    ) -> Self {
        Self {
            id: id.to_string(),
            fixture_id: fixture_id.to_string(),
            original_year,
            original_day_of_year: original_day_of_year as i32,
            new_year,
            new_day_of_year: new_day_of_year as i32,
            reason: reason.into(),
            created_at_unix_seconds,
        }
    }
}
