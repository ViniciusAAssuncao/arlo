use crate::error::DbResult;
use arlo_domain::{AttributeDefinition, PlayerAttributeValue};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct PlayerAttributeRow {
    pub player_id: String,
    pub attribute_definition_id: String,
    pub value: i32,
}

impl PlayerAttributeRow {
    pub fn to_domain(&self, definition: &AttributeDefinition) -> DbResult<PlayerAttributeValue> {
        PlayerAttributeValue::new(definition, self.value).map_err(Into::into)
    }
}
