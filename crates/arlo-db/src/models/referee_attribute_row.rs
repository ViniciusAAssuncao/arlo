use crate::error::DbResult;
use arlo_domain::{AttributeDefinition, RefereeAttributeValue};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct RefereeAttributeRow {
    pub referee_id: String,
    pub attribute_definition_id: String,
    pub value: i32,
}

impl RefereeAttributeRow {
    pub fn to_domain(&self, definition: &AttributeDefinition) -> DbResult<RefereeAttributeValue> {
        RefereeAttributeValue::new(definition, self.value).map_err(Into::into)
    }
}