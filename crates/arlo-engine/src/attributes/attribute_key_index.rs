use arlo_domain::AttributeKey;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeKeyIndex {
    keys: [Option<Uuid>; AttributeKey::COUNT],
}

impl AttributeKeyIndex {
    pub fn from_map(attribute_keys: &HashMap<Uuid, AttributeKey>) -> Self {
        let mut keys = [None; AttributeKey::COUNT];
        for (&uuid, &key) in attribute_keys {
            keys[key.index()] = Some(uuid);
        }
        Self { keys }
    }

    pub fn get(&self, key: AttributeKey) -> Option<Uuid> {
        self.keys[key.index()]
    }
}

impl Default for AttributeKeyIndex {
    fn default() -> Self {
        Self {
            keys: [None; AttributeKey::COUNT],
        }
    }
}

impl Serialize for AttributeKeyIndex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.keys.as_slice().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AttributeKeyIndex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec = Vec::<Option<Uuid>>::deserialize(deserializer)?;
        let mut keys = [None; AttributeKey::COUNT];
        for (i, val) in vec.into_iter().enumerate().take(AttributeKey::COUNT) {
            keys[i] = val;
        }
        Ok(Self { keys })
    }
}
