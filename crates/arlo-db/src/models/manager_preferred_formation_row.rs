use crate::error::DbResult;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct ManagerPreferredFormationRow {
    pub id: String,
    pub manager_tactical_profile_id: String,
    pub formation_id: String,
}

impl ManagerPreferredFormationRow {
    pub fn formation_id(&self) -> DbResult<Uuid> {
        Uuid::parse_str(&self.formation_id).map_err(Into::into)
    }
}