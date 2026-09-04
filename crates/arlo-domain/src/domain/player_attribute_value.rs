use crate::domain::attribute_definition::{ AttributeDefinition, AttributeTarget };
use crate::domain::validation::validate_integer_range;
use crate::error::{ DomainError, DomainResult };
use serde::{ Deserialize, Serialize };
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerAttributeValue {
    attribute_definition_id: Uuid,
    value: i32,
}

impl PlayerAttributeValue {
    pub fn new(definition: &AttributeDefinition, value: i32) -> DomainResult<Self> {
        if definition.applies_to() != AttributeTarget::Player {
            return Err(DomainError::InvalidInvariant {
                field: "attribute_definition".to_string(),
                reason: "attribute definition does not apply to player".to_string(),
            });
        }
        validate_integer_range(value, 0, 20, "value")?;

        Ok(Self {
            attribute_definition_id: definition.id(),
            value,
        })
    }

    pub fn attribute_definition_id(&self) -> Uuid {
        self.attribute_definition_id
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
