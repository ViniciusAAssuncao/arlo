use crate::error::DbResult;
use crate::models::position_code::parse_position;
use arlo_domain::PlayerPosition;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct PlayerPositionRow {
    pub player_id: String,
    pub position: String,
    pub proficiency: i32,
}

impl PlayerPositionRow {
    pub fn to_domain(&self) -> DbResult<PlayerPosition> {
        let position = parse_position(&self.position)?;
        PlayerPosition::new(position, self.proficiency).map_err(Into::into)
    }
}