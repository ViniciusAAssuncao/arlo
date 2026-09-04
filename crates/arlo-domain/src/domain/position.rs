use crate::domain::validation::validate_not_empty;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    id: Uuid,
    code: String,
    name: String,
}

impl Position {
    pub fn new(
        id: Uuid,
        code: impl Into<String>,
        name: impl Into<String>,
    ) -> DomainResult<Self> {
        let code = code.into();
        let name = name.into();
        validate_not_empty(&code, "code")?;
        validate_not_empty(&name, "name")?;

        Ok(Self { id, code, name })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
