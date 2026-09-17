pub mod defense_duels;
pub mod goalkeeping_duels;
pub mod offense_duels;

pub use defense_duels::*;
pub use goalkeeping_duels::*;
pub use offense_duels::*;

use crate::attributes::PlayerAttributeTable;
use crate::caching::duel_profile_cache::get_cached_duel_profiles;
use crate::resolution::duel_kind::DuelKind;
use crate::weighting::AttributeWeight;
use serde::{Deserialize, Serialize};

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
        let mut total_weight = 0.0;
        let mut weighted_sum = 0.0;
        for w in &self.weights {
            if w.weight > 0.0 {
                weighted_sum += table.get(w.key) * w.weight;
                total_weight += w.weight;
            }
        }
        if total_weight > 0.0 {
            weighted_sum / total_weight
        } else {
            0.0
        }
    }
}

pub fn get_duel_profiles(kind: DuelKind) -> &'static (DuelProfile, DuelProfile) {
    get_cached_duel_profiles(kind)
}

pub fn get_duel_profile_weights(
    kind: DuelKind,
) -> (&'static [AttributeWeight], &'static [AttributeWeight]) {
    let (att, def) = get_cached_duel_profiles(kind);
    (att.weights(), def.weights())
}
