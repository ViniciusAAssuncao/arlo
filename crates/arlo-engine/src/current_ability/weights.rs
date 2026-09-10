pub use crate::weighting::AttributeWeight;
use arlo_domain::{AttributeKey, Position};

#[derive(Debug, Clone, PartialEq)]
pub struct PositionWeightProfile {
    pub position: Position,
    pub weights: Vec<AttributeWeight>,
    pub dense_weights: [Option<f64>; AttributeKey::COUNT],
}

impl PositionWeightProfile {
    pub fn new(position: Position, weights: Vec<AttributeWeight>) -> Self {
        let mut dense_weights = [None; AttributeKey::COUNT];
        for w in &weights {
            if w.weight > 0.0 {
                dense_weights[w.key.index()] = Some(w.weight);
            }
        }
        Self {
            position,
            weights,
            dense_weights,
        }
    }

    pub fn weight_for(&self, key: AttributeKey) -> Option<f64> {
        self.dense_weights[key.index()]
    }
}