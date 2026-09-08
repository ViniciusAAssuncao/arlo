use crate::error::TacticsResult;
use crate::lineup::{SlotAssignment, TacticalLineup};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct TacticalLineupRow {
    pub id: String,
    pub team_id: String,
    pub formation_id: String,
    pub name: String,
    pub created_at_unix_seconds: i64,
}

impl TacticalLineupRow {
    pub fn to_domain(&self, assignments: Vec<SlotAssignment>) -> TacticsResult<TacticalLineup> {
        let id = Uuid::parse_str(&self.id)?;
        let team_id = Uuid::parse_str(&self.team_id)?;
        let formation_id = Uuid::parse_str(&self.formation_id)?;
        Ok(TacticalLineup::new(
            id,
            team_id,
            formation_id,
            &self.name,
            assignments,
        ))
    }
}