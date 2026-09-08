use crate::error::DbResult;
use arlo_domain::{Manager, ManagerAttributeValue, ManagerTacticalProfile, Person};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct ManagerRow {
    pub id: String,
    pub team_id: Option<String>,
}

impl ManagerRow {
    pub fn to_domain(
        &self,
        person: Person,
        attributes: Vec<ManagerAttributeValue>,
        tactical_profile: Option<ManagerTacticalProfile>,
    ) -> DbResult<Manager> {
        let team_id = match &self.team_id {
            Some(tid) => Some(Uuid::parse_str(tid)?),
            None => None,
        };
        Manager::new(person, team_id, attributes, tactical_profile).map_err(Into::into)
    }
}