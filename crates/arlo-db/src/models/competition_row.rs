use crate::error::{DbError, DbResult};
use arlo_domain::{Competition, CompetitionKind, Scope};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct CompetitionRow {
    pub id: String,
    pub name: String,
    pub federation_id: String,
    pub country_id: Option<String>,
    pub scope: String,
    pub kind: String,
    pub prestige: i32,
}

impl CompetitionRow {
    pub fn to_domain(&self) -> DbResult<Competition> {
        let id = Uuid::parse_str(&self.id)?;
        let federation_id = Uuid::parse_str(&self.federation_id)?;
        let country_id = match &self.country_id {
            Some(cid) => Some(Uuid::parse_str(cid)?),
            None => None,
        };
        let scope = match self.scope.as_str() {
            "Regional" => Scope::Regional,
            "National" => Scope::National,
            "Continental" => Scope::Continental,
            "International" => Scope::International,
            _ => return Err(DbError::InvalidEnum(format!("Invalid scope: {}", self.scope))),
        };
        let kind = match self.kind.as_str() {
            "League" => CompetitionKind::League,
            "Cup" => CompetitionKind::Cup,
            "Friendly" => CompetitionKind::Friendly,
            _ => {
                return Err(DbError::InvalidEnum(format!(
                    "Invalid competition kind: {}",
                    self.kind
                )))
            }
        };
        Competition::new(
            id,
            &self.name,
            federation_id,
            country_id,
            scope,
            kind,
            self.prestige,
        )
        .map_err(Into::into)
    }
}