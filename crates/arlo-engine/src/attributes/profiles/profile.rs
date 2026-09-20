use crate::attributes::PlayerAttributeTable;
use crate::weighting::aggregate::{calculate_weighted_average, calculate_weighted_saturated_average};
use crate::weighting::AttributeWeight;
use arlo_domain::sport_constants::{
    ATTRIBUTE_SATURATION_MULTIPLIER, ATTRIBUTE_SATURATION_THRESHOLD,
};
use arlo_domain::AttributeKey;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeProfile {
    weights: Vec<AttributeWeight>,
}

impl AttributeProfile {
    pub fn new(weights: Vec<AttributeWeight>) -> Self {
        Self { weights }
    }

    pub fn weights(&self) -> &[AttributeWeight] {
        &self.weights
    }

    pub fn weight_for(&self, key: AttributeKey) -> Option<f64> {
        self.weights
            .iter()
            .find(|w| w.key == key)
            .map(|w| w.weight)
    }

    pub fn evaluate_weighted_average<F>(&self, value_fn: F) -> f64
    where
        F: Fn(AttributeKey) -> f64,
    {
        let items: SmallVec<[(f64, f64); 16]> = self
            .weights
            .iter()
            .filter(|w| w.weight > 0.0)
            .map(|w| (value_fn(w.key), w.weight))
            .collect();
        calculate_weighted_average(&items).unwrap_or(10.0)
    }

    pub fn evaluate_saturated_average<F>(&self, value_fn: F) -> f64
    where
        F: Fn(AttributeKey) -> f64,
    {
        let items: SmallVec<[(f64, f64); 16]> = self
            .weights
            .iter()
            .filter(|w| w.weight > 0.0)
            .map(|w| (value_fn(w.key), w.weight))
            .collect();
        calculate_weighted_saturated_average(
            &items,
            ATTRIBUTE_SATURATION_THRESHOLD,
            ATTRIBUTE_SATURATION_MULTIPLIER,
        )
        .unwrap_or(10.0)
    }

    pub fn rate(&self, table: &PlayerAttributeTable) -> f64 {
        self.evaluate_weighted_average(|key| table.get(key))
    }
}

pub fn w(key: AttributeKey, weight: f64) -> AttributeWeight {
    AttributeWeight::new(key, weight)
}
