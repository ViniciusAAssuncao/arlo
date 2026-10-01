use crate::error::DbResult;
use arlo_domain::CompetitionGroup;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct CompetitionGroupRow {
    pub id: String,
    pub league_calendar_config_id: String,
    pub order_index: i32,
    pub name: String,
}

impl CompetitionGroupRow {
    pub fn to_domain(&self, team_ids: Vec<Uuid>) -> DbResult<CompetitionGroup> {
        let id = Uuid::parse_str(&self.id)?;
        CompetitionGroup::new(id, self.order_index as u32, &self.name, team_ids).map_err(Into::into)
    }
}
