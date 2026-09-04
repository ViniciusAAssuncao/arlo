use crate::domain::validation::validate_not_empty;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Country {
    id: Uuid,
    name: String,
    continent_id: Uuid,
    federation_id: Option<Uuid>,
}

impl Country {
    pub fn new(
        id: Uuid,
        name: impl Into<String>,
        continent_id: Uuid,
        federation_id: Option<Uuid>,
    ) -> DomainResult<Self> {
        let name = name.into();
        validate_not_empty(&name, "name")?;
        Ok(Self {
            id,
            name,
            continent_id,
            federation_id,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn continent_id(&self) -> Uuid {
        self.continent_id
    }

    pub fn federation_id(&self) -> Option<Uuid> {
        self.federation_id
    }
}
