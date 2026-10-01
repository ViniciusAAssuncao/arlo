use crate::error::DbResult;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct CompetitionGroupTeamRow {
    pub id: String,
    pub competition_group_id: String,
    pub team_id: String,
}

impl CompetitionGroupTeamRow {
    pub fn team_id(&self) -> DbResult<Uuid> {
        Uuid::parse_str(&self.team_id).map_err(Into::into)
    }
}
