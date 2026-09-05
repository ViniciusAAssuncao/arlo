use crate::error::DbResult;
use arlo_domain::{AttributeDefinition, ManagerAttributeValue};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct ManagerAttributeRow {
    pub manager_id: String,
    pub attribute_definition_id: String,
    pub value: i32,
}

impl ManagerAttributeRow {
    pub fn to_domain(&self, definition: &AttributeDefinition) -> DbResult<ManagerAttributeValue> {
        ManagerAttributeValue::new(definition, self.value).map_err(Into::into)
    }
}