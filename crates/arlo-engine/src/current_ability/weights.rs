use arlo_domain::{AttributeKey, Position};

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeWeight {
    pub key: AttributeKey,
    pub weight: f64,
}

impl AttributeWeight {
    pub fn new(key: AttributeKey, weight: f64) -> Self {
        Self { key, weight }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PositionWeightProfile {
    pub position: Position,
    pub weights: Vec<AttributeWeight>,
}

impl PositionWeightProfile {
    pub fn new(position: Position, weights: Vec<AttributeWeight>) -> Self {
        Self { position, weights }
    }
}
