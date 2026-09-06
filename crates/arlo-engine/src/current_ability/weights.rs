pub use crate::weighting::AttributeWeight;
use arlo_domain::Position;

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