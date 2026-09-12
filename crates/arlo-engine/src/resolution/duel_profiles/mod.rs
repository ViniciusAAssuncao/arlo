pub mod defense_duels;
pub mod goalkeeping_duels;
pub mod offense_duels;

pub use defense_duels::*;
pub use goalkeeping_duels::*;
pub use offense_duels::*;

use crate::attributes::PlayerAttributeTable;
use crate::caching::duel_profile_cache::get_cached_duel_profiles;
use crate::resolution::duel_kind::DuelKind;
use crate::weighting::{calculate_weighted_average, AttributeWeight};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DuelProfile {
    weights: Vec<AttributeWeight>,
}

impl DuelProfile {
    pub fn new(weights: Vec<AttributeWeight>) -> Self {
        Self { weights }
    }

    pub fn weights(&self) -> &[AttributeWeight] {
        &self.weights
    }

    pub fn rate(&self, table: &PlayerAttributeTable) -> f64 {
        let pairs: SmallVec<[(f64, f64); 8]> = self
            .weights
            .iter()
            .filter(|w| w.weight > 0.0)
            .map(|w| (table.get(w.key), w.weight))
            .collect();
        calculate_weighted_average(&pairs).unwrap_or(0.0)
    }
}

pub fn get_duel_profiles(kind: DuelKind) -> &'static (DuelProfile, DuelProfile) {
    get_cached_duel_profiles(kind)
}