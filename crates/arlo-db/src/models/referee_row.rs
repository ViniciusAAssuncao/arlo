use crate::error::DbResult;
use crate::models::scope_code::parse_scope;
use arlo_domain::{Person, Referee, RefereeAttributeValue};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct RefereeRow {
    pub id: String,
    pub primary_league_id: Option<String>,
    pub tier: String,
}

impl RefereeRow {
    pub fn to_domain(
        &self,
        person: Person,
        attributes: Vec<RefereeAttributeValue>,
    ) -> DbResult<Referee> {
        let primary_league_id = match &self.primary_league_id {
            Some(lid) => Some(Uuid::parse_str(lid)?),
            None => None,
        };
        let tier = parse_scope(&self.tier)?;
        Referee::new(person, primary_league_id, tier, attributes).map_err(Into::into)
    }
}