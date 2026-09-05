use crate::error::{DbError, DbResult};
use arlo_domain::{Federation, Scope};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct FederationRow {
    pub id: String,
    pub name: String,
    pub scope: String,
    pub continent_id: Option<String>,
    pub parent_federation_id: Option<String>,
    pub prestige: i32,
}

impl FederationRow {
    pub fn to_domain(&self) -> DbResult<Federation> {
        let id = Uuid::parse_str(&self.id)?;
        let scope = match self.scope.as_str() {
            "Regional" => Scope::Regional,
            "National" => Scope::National,
            "Continental" => Scope::Continental,
            "International" => Scope::International,
            _ => return Err(DbError::InvalidEnum(format!("Invalid scope: {}", self.scope))),
        };
        let continent_id = match &self.continent_id {
            Some(cid) => Some(Uuid::parse_str(cid)?),
            None => None,
        };
        let parent_federation_id = match &self.parent_federation_id {
            Some(pid) => Some(Uuid::parse_str(pid)?),
            None => None,
        };
        Federation::new(
            id,
            &self.name,
            scope,
            continent_id,
            parent_federation_id,
            self.prestige,
        )
        .map_err(Into::into)
    }
}