use crate::domain::validation::{validate_not_empty, validate_positive_finite};
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    id: Uuid,
    name: String,
    height_m: f64,
    birthdate_unix_seconds: i64,
    nationality_id: Uuid,
}

impl Person {
    pub fn new(
        id: Uuid,
        name: impl Into<String>,
        height_m: f64,
        birthdate_unix_seconds: i64,
        nationality_id: Uuid,
    ) -> DomainResult<Self> {
        let name = name.into();
        validate_not_empty(&name, "name")?;
        validate_positive_finite(height_m, "height_m")?;

        Ok(Self {
            id,
            name,
            height_m,
            birthdate_unix_seconds,
            nationality_id,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn height_m(&self) -> f64 {
        self.height_m
    }

    pub fn birthdate_unix_seconds(&self) -> i64 {
        self.birthdate_unix_seconds
    }

    pub fn nationality_id(&self) -> Uuid {
        self.nationality_id
    }
}
