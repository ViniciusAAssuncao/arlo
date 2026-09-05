use crate::error::{DbError, DbResult};
use arlo_domain::{AttributeCategory, AttributeDefinition, AttributeTarget};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct AttributeDefinitionRow {
    pub id: String,
    pub key: String,
    pub display_name: String,
    pub category: String,
    pub applies_to: String,
}

impl AttributeDefinitionRow {
    pub fn to_domain(&self) -> DbResult<AttributeDefinition> {
        let id = Uuid::parse_str(&self.id)?;
        let category = match self.category.as_str() {
            "Technical" => AttributeCategory::Technical,
            "Mental" => AttributeCategory::Mental,
            "Physical" => AttributeCategory::Physical,
            "Tactical" => AttributeCategory::Tactical,
            "Managerial" => AttributeCategory::Managerial,
            "Goalkeeping" => AttributeCategory::Goalkeeping,
            _ => {
                return Err(DbError::InvalidEnum(format!(
                    "Invalid attribute category: {}",
                    self.category
                )))
            }
        };
        let applies_to = match self.applies_to.as_str() {
            "Player" => AttributeTarget::Player,
            "Manager" => AttributeTarget::Manager,
            _ => {
                return Err(DbError::InvalidEnum(format!(
                    "Invalid attribute target: {}",
                    self.applies_to
                )))
            }
        };
        AttributeDefinition::new(id, &self.key, &self.display_name, category, applies_to)
            .map_err(Into::into)
    }
}