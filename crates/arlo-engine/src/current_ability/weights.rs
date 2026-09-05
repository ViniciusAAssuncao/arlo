use arlo_domain::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeWeight {
    pub key: String,
    pub weight: f64,
}

impl AttributeWeight {
    pub fn new(key: impl Into<String>, weight: f64) -> Self {
        Self {
            key: key.into(),
            weight,
        }
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