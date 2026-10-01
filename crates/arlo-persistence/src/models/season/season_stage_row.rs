use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct SeasonStageRow {
    pub id: String,
    pub season_instance_id: String,
    pub stage_order_index: i32,
    pub stage_type: String,
    pub status: String,
}

impl SeasonStageRow {
    pub fn new(
        id: Uuid,
        season_instance_id: Uuid,
        stage_order_index: u32,
        stage_type: impl Into<String>,
        status: impl Into<String>,
    ) -> Self {
        Self {
            id: id.to_string(),
            season_instance_id: season_instance_id.to_string(),
            stage_order_index: stage_order_index as i32,
            stage_type: stage_type.into(),
            status: status.into(),
        }
    }
}
