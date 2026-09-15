use crate::error::DbResult;
use crate::models::manager_control_mode_code::parse_manager_control_mode;
use arlo_domain::{Manager, ManagerAttributeValue, ManagerTacticalProfile, Person};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct ManagerRow {
    pub id: String,
    pub team_id: Option<String>,
    pub control_mode: Option<String>,
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
        let control_mode = parse_manager_control_mode(self.control_mode.as_deref());
        Manager::new(
            person,
            team_id,
            control_mode,
            attributes,
            tactical_profile,
        )
        .map_err(Into::into)
    }
}