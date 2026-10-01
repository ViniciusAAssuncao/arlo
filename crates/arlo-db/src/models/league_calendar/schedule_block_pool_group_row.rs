use crate::error::DbResult;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct ScheduleBlockPoolGroupRow {
    pub id: String,
    pub schedule_block_id: String,
    pub group_id: String,
}

impl ScheduleBlockPoolGroupRow {
    pub fn group_id(&self) -> DbResult<Uuid> {
        Uuid::parse_str(&self.group_id).map_err(Into::into)
    }
}
