use crate::error::DbResult;
use arlo_domain::Position;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct PositionRow {
    pub id: String,
    pub code: String,
    pub name: String,
}

impl PositionRow {
    pub fn to_domain(&self) -> DbResult<Position> {
        let id = Uuid::parse_str(&self.id)?;
        Position::new(id, &self.code, &self.name).map_err(Into::into)
    }
}