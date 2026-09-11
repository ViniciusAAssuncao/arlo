use crate::domain::fault_severity::FaultSeverity;
use crate::domain::validation::validate_not_empty;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FaultDefinition {
    id: Uuid,
    code: String,
    description: String,
    severity: FaultSeverity,
}

impl FaultDefinition {
    pub fn new(
        id: Uuid,
        code: impl Into<String>,
        description: impl Into<String>,
        severity: FaultSeverity,
    ) -> DomainResult<Self> {
        let code = code.into();
        let description = description.into();
        validate_not_empty(&code, "code")?;
        validate_not_empty(&description, "description")?;

        Ok(Self {
            id,
            code,
            description,
            severity,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn severity(&self) -> FaultSeverity {
        self.severity
    }
}