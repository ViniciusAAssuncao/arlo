use crate::error::DbResult;
use arlo_domain::Team;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct TeamRow {
    pub id: String,
    pub name: String,
    pub country_id: String,
    pub league_id: Option<String>,
    pub founded_at_unix_seconds: i64,
    pub prestige: i32,
    pub primary_color_hex: Option<String>,
    pub secondary_color_hex: Option<String>,
    pub home_venue_id: Option<String>,
}

impl TeamRow {
    pub fn to_domain(&self) -> DbResult<Team> {
        let id = Uuid::parse_str(&self.id)?;
        let country_id = Uuid::parse_str(&self.country_id)?;
        let league_id = match &self.league_id {
            Some(lid) => Some(Uuid::parse_str(lid)?),
            None => None,
        };
        let home_venue_id = match &self.home_venue_id {
            Some(vid) => Some(Uuid::parse_str(vid)?),
            None => None,
        };
        let mut builder = Team::builder(
            id,
            &self.name,
            country_id,
            self.founded_at_unix_seconds,
            self.prestige,
        );
        builder = builder.with_league_id(league_id);
        builder = builder.with_primary_color_hex(self.primary_color_hex.clone());
        builder = builder.with_secondary_color_hex(self.secondary_color_hex.clone());
        builder = builder.with_home_venue_id(home_venue_id);
        builder.build().map_err(Into::into)
    }
}
