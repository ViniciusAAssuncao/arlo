use crate::attributes::ManagerAttributeTable;
use arlo_domain::{AttributeKey, Manager};
use std::collections::HashMap;
use uuid::Uuid;

pub fn extract_manager_attribute_value(
    manager: &Manager,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    key: AttributeKey,
) -> f64 {
    let table = ManagerAttributeTable::from_manager(manager, attribute_keys);
    table.get(key)
}

pub fn extract_manager_attribute_value_from_table(
    table: &ManagerAttributeTable,
    key: AttributeKey,
) -> f64 {
    table.get(key)
}
