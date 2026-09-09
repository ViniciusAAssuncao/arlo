use crate::world_state::constants::DEFAULT_ATTRIBUTE_VALUE;
use arlo_domain::{AttributeKey, Manager};
use std::collections::HashMap;
use uuid::Uuid;

pub fn extract_manager_attribute_value(
    manager: &Manager,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    key: AttributeKey,
) -> f64 {
    for attr in manager.attributes() {
        if let Some(&attr_key) = attribute_keys.get(&attr.attribute_definition_id()) {
            if attr_key == key {
                return attr.value() as f64;
            }
        }
    }
    DEFAULT_ATTRIBUTE_VALUE
}
