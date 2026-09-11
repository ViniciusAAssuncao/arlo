use crate::error::DbResult;
use crate::models::scope_code::parse_scope;
use arlo_domain::Federation;
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
        let scope = parse_scope(&self.scope)?;
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