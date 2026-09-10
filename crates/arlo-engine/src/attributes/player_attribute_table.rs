use crate::attributes::attribute_key_index::AttributeKeyIndex;
use arlo_domain::{AttributeKey, Player};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerAttributeTable {
    values: [f64; AttributeKey::COUNT],
}

impl PlayerAttributeTable {
    pub fn new(values: [f64; AttributeKey::COUNT]) -> Self {
        Self { values }
    }

    pub const fn new_default() -> Self {
        Self {
            values: [10.0; AttributeKey::COUNT],
        }
    }

    pub fn from_player(
        player: &Player,
        attribute_keys: &HashMap<Uuid, AttributeKey>,
    ) -> Self {
        let mut values = [10.0; AttributeKey::COUNT];
        for attr in player.attributes() {
            if let Some(&key) = attribute_keys.get(&attr.attribute_definition_id()) {
                values[key.index()] = attr.value() as f64;
            }
        }
        Self { values }
    }

    pub fn from_player_with_index(
        player: &Player,
        key_index: &AttributeKeyIndex,
    ) -> Self {
        let mut values = [10.0; AttributeKey::COUNT];
        for key in AttributeKey::all() {
            if let Some(target_id) = key_index.get(key) {
                for attr in player.attributes() {
                    if attr.attribute_definition_id() == target_id {
                        values[key.index()] = attr.value() as f64;
                        break;
                    }
                }
            }
        }
        Self { values }
    }

    pub fn get(&self, key: AttributeKey) -> f64 {
        self.values[key.index()]
    }

    pub fn set(&mut self, key: AttributeKey, value: f64) {
        self.values[key.index()] = value;
    }

    pub fn values(&self) -> &[f64; AttributeKey::COUNT] {
        &self.values
    }
}

impl Default for PlayerAttributeTable {
    fn default() -> Self {
        Self::new_default()
    }
}

impl Serialize for PlayerAttributeTable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.values.as_slice().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PlayerAttributeTable {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec = Vec::<f64>::deserialize(deserializer)?;
        let mut values = [10.0; AttributeKey::COUNT];
        for (i, val) in vec.into_iter().enumerate().take(AttributeKey::COUNT) {
            values[i] = val;
        }
        Ok(Self { values })
    }
}