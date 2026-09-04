use crate::domain::scope::Scope;
use crate::domain::validation::{validate_integer_range, validate_not_empty};
use crate::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompetitionKind {
    League,
    Cup,
    Friendly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Competition {
    id: Uuid,
    name: String,
    federation_id: Uuid,
    country_id: Option<Uuid>,
    scope: Scope,
    kind: CompetitionKind,
    prestige: i32,
}

impl Competition {
    pub fn new(
        id: Uuid,
        name: impl Into<String>,
        federation_id: Uuid,
        country_id: Option<Uuid>,
        scope: Scope,
        kind: CompetitionKind,
        prestige: i32,
    ) -> DomainResult<Self> {
        let name = name.into();
        validate_not_empty(&name, "name")?;
        validate_integer_range(prestige, 0, 200, "prestige")?;

        match scope {
            Scope::Regional | Scope::National => {
                if country_id.is_none() {
                    return Err(DomainError::InvalidInvariant {
                        field: "country_id".to_string(),
                        reason: "country_id must be present for Regional or National scope"
                            .to_string(),
                    });
                }
            }
            Scope::Continental | Scope::International => {
                if country_id.is_some() {
                    return Err(DomainError::InvalidInvariant {
                        field: "country_id".to_string(),
                        reason: "country_id must be None for Continental or International scope"
                            .to_string(),
                    });
                }
            }
        }

        Ok(Self {
            id,
            name,
            federation_id,
            country_id,
            scope,
            kind,
            prestige,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn federation_id(&self) -> Uuid {
        self.federation_id
    }

    pub fn country_id(&self) -> Option<Uuid> {
        self.country_id
    }

    pub fn scope(&self) -> Scope {
        self.scope
    }

    pub fn kind(&self) -> CompetitionKind {
        self.kind
    }

    pub fn prestige(&self) -> i32 {
        self.prestige
    }
}
