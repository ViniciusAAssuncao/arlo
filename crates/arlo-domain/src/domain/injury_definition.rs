use crate::domain::body_region::BodyRegion;
use crate::domain::injury_mechanism::InjuryMechanism;
use crate::domain::validation::{validate_not_empty, validate_positive_finite};
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InjuryDefinition {
    id: Uuid,
    code: String,
    description: String,
    mechanism: InjuryMechanism,
    body_region: BodyRegion,
    relative_frequency: f64,
}

impl InjuryDefinition {
    pub fn new(
        id: Uuid,
        code: impl Into<String>,
        description: impl Into<String>,
        mechanism: InjuryMechanism,
        body_region: BodyRegion,
        relative_frequency: f64,
    ) -> DomainResult<Self> {
        let code = code.into();
        let description = description.into();
        validate_not_empty(&code, "code")?;
        validate_not_empty(&description, "description")?;
        validate_positive_finite(relative_frequency, "relative_frequency")?;

        Ok(Self {
            id,
            code,
            description,
            mechanism,
            body_region,
            relative_frequency,
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

    pub fn mechanism(&self) -> InjuryMechanism {
        self.mechanism
    }

    pub fn body_region(&self) -> BodyRegion {
        self.body_region
    }

    pub fn relative_frequency(&self) -> f64 {
        self.relative_frequency
    }
}