use crate::domain::calendar::calendar_system::CalendarSystem;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CalendarCatalog {
    definitions_by_id: HashMap<Uuid, CalendarSystem>,
}

impl CalendarCatalog {
    pub fn new(definitions: Vec<CalendarSystem>) -> Self {
        let mut definitions_by_id = HashMap::with_capacity(definitions.len());
        for def in definitions {
            definitions_by_id.insert(def.id(), def);
        }
        Self { definitions_by_id }
    }

    pub fn get(&self, id: &Uuid) -> Option<&CalendarSystem> {
        self.definitions_by_id.get(id)
    }

    pub fn all(&self) -> impl Iterator<Item = &CalendarSystem> {
        self.definitions_by_id.values()
    }

    pub fn definitions_by_id(&self) -> &HashMap<Uuid, CalendarSystem> {
        &self.definitions_by_id
    }
}