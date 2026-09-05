use crate::error::DbResult;
use arlo_domain::{Formation, FormationSlot};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct FormationRow {
    pub id: String,
    pub name: String,
}

impl FormationRow {
    pub fn to_domain(&self, slots: Vec<FormationSlot>) -> DbResult<Formation> {
        let id = Uuid::parse_str(&self.id)?;
        Formation::new(id, &self.name, slots).map_err(Into::into)
    }
}
