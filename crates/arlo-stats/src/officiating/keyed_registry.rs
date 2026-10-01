use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub trait KeyedStat {
    fn new_for(player_id: Uuid) -> Self;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyedStatRegistry<T> {
    stats: HashMap<Uuid, T>,
}

impl<T> Default for KeyedStatRegistry<T> {
    fn default() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }
}

impl<T: KeyedStat + Clone> KeyedStatRegistry<T> {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    pub fn get(&self, id: &Uuid) -> Option<&T> {
        self.stats.get(id)
    }

    pub fn get_or_default(&self, id: &Uuid) -> T {
        self.stats
            .get(id)
            .cloned()
            .unwrap_or_else(|| T::new_for(*id))
    }

    pub fn all_stats(&self) -> &HashMap<Uuid, T> {
        &self.stats
    }

    pub fn entry_or_default(&mut self, id: Uuid) -> &mut T {
        self.stats.entry(id).or_insert_with(|| T::new_for(id))
    }

    pub fn clear(&mut self) {
        self.stats.clear();
    }
}
