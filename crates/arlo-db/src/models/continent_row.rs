use crate::error::DbResult;
use arlo_domain::Continent;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct ContinentRow {
    pub id: String,
    pub name: String,
}

impl ContinentRow {
    pub fn to_domain(&self) -> DbResult<Continent> {
        let id = Uuid::parse_str(&self.id)?;
        Continent::new(id, &self.name).map_err(Into::into)
    }
}