use crate::domain::invariant_violation::InvariantViolation;
use crate::domain::scope::Scope;
use crate::domain::validation::{validate_integer_range, validate_not_empty};
use crate::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Federation {
    id: Uuid,
    name: String,
    scope: Scope,
    continent_id: Option<Uuid>,
    parent_federation_id: Option<Uuid>,
    prestige: i32,
}

impl Federation {
    pub fn new(
        id: Uuid,
        name: impl Into<String>,
        scope: Scope,
        continent_id: Option<Uuid>,
        parent_federation_id: Option<Uuid>,
        prestige: i32,
    ) -> DomainResult<Self> {
        let name = name.into();
        validate_not_empty(&name, "name")?;
        validate_integer_range(prestige, 0, 200, "prestige")?;

        match (scope, continent_id) {
            (Scope::Continental, None) => {
                return Err(DomainError::InvalidInvariant {
                    field: "continent_id".to_string(),
                    violation: InvariantViolation::MissingRequiredValue,
                });
            }
            (s, Some(_)) if s != Scope::Continental => {
                return Err(DomainError::InvalidInvariant {
                    field: "continent_id".to_string(),
                    violation: InvariantViolation::UnexpectedValue,
                });
            }
            _ => {}
        }

        if parent_federation_id == Some(id) {
            return Err(DomainError::InvalidInvariant {
                field: "parent_federation_id".to_string(),
                violation: InvariantViolation::SelfReference,
            });
        }

        Ok(Self {
            id,
            name,
            scope,
            continent_id,
            parent_federation_id,
            prestige,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn scope(&self) -> Scope {
        self.scope
    }

    pub fn continent_id(&self) -> Option<Uuid> {
        self.continent_id
    }

    pub fn parent_federation_id(&self) -> Option<Uuid> {
        self.parent_federation_id
    }

    pub fn prestige(&self) -> i32 {
        self.prestige
    }
}
