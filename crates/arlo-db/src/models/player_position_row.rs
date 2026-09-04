use crate::error::DbResult;
use arlo_domain::PlayerPosition;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct PlayerPositionRow {
    pub player_id: String,
    pub position_id: String,
    pub proficiency: i32,
}

impl PlayerPositionRow {
    pub fn to_domain(&self) -> DbResult<PlayerPosition> {
        let position_id = Uuid::parse_str(&self.position_id)?;
        PlayerPosition::new(position_id, self.proficiency).map_err(Into::into)
    }
}