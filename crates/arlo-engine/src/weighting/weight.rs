use arlo_domain::AttributeKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AttributeWeight {
    pub key: AttributeKey,
    pub weight: f64,
}

impl AttributeWeight {
    pub fn new(key: AttributeKey, weight: f64) -> Self {
        Self { key, weight }
    }
}
