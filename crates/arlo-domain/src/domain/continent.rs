use crate::domain::validation::validate_not_empty;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Continent {
    id: Uuid,
    name: String,
}

impl Continent {
    pub fn new(id: Uuid, name: impl Into<String>) -> DomainResult<Self> {
        let name = name.into();
        validate_not_empty(&name, "name")?;
        Ok(Self { id, name })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
