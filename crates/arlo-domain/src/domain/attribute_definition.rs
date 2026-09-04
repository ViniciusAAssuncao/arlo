use crate::domain::validation::validate_not_empty;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttributeCategory {
    Technical,
    Mental,
    Physical,
    Tactical,
    Managerial,
    Goalkeeping,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttributeTarget {
    Player,
    Manager,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttributeDefinition {
    id: Uuid,
    key: String,
    display_name: String,
    category: AttributeCategory,
    applies_to: AttributeTarget,
}

impl AttributeDefinition {
    pub fn new(
        id: Uuid,
        key: impl Into<String>,
        display_name: impl Into<String>,
        category: AttributeCategory,
        applies_to: AttributeTarget,
    ) -> DomainResult<Self> {
        let key = key.into();
        let display_name = display_name.into();
        validate_not_empty(&key, "key")?;
        validate_not_empty(&display_name, "display_name")?;

        Ok(Self {
            id,
            key,
            display_name,
            category,
            applies_to,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn category(&self) -> AttributeCategory {
        self.category
    }

    pub fn applies_to(&self) -> AttributeTarget {
        self.applies_to
    }
}