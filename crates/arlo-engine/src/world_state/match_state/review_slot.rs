use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewSlot<T> {
    entry: Option<(Uuid, T)>,
}

impl<T> ReviewSlot<T> {
    pub fn new() -> Self {
        Self { entry: None }
    }

    pub fn entry(&self) -> Option<&(Uuid, T)> {
        self.entry.as_ref()
    }

    pub fn set(&mut self, team_id: Uuid, value: T) {
        self.entry = Some((team_id, value));
    }

    pub fn clear(&mut self) {
        self.entry = None;
    }
}

impl<T> Default for ReviewSlot<T> {
    fn default() -> Self {
        Self { entry: None }
    }
}