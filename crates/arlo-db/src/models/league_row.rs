use crate::error::DbResult;
use arlo_domain::{Competition, League};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct LeagueRow {
    pub competition_id: String,
    pub division_index: i32,
}

impl LeagueRow {
    pub fn to_domain(&self, competition: Competition) -> DbResult<League> {
        Ok(League::new(competition, self.division_index as u32))
    }
}
