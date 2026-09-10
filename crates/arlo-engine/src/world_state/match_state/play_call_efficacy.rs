use arlo_math::stats::BetaBelief;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayCallEfficacyTracker {
    home: HashMap<Uuid, BetaBelief>,
    away: HashMap<Uuid, BetaBelief>,
}

impl PlayCallEfficacyTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn home(&self) -> &HashMap<Uuid, BetaBelief> {
        &self.home
    }

    pub fn away(&self) -> &HashMap<Uuid, BetaBelief> {
        &self.away
    }

    pub fn get_or_seed(
        &mut self,
        is_home: bool,
        play_call_id: Uuid,
        prior_mean: f64,
        prior_strength: f64,
    ) -> BetaBelief {
        let map = if is_home {
            &mut self.home
        } else {
            &mut self.away
        };
        *map.entry(play_call_id)
            .or_insert_with(|| BetaBelief::from_prior_mean_and_strength(prior_mean, prior_strength))
    }

    pub fn record_outcome(
        &mut self,
        is_home: bool,
        play_call_id: Uuid,
        success: bool,
        decay_factor: f64,
    ) {
        let map = if is_home {
            &mut self.home
        } else {
            &mut self.away
        };
        let belief = map
            .entry(play_call_id)
            .or_insert_with(|| BetaBelief::from_prior_mean_and_strength(0.5, 2.0));
        belief.update_with_decay(success, decay_factor);
    }

    pub fn snapshot(&self, is_home: bool) -> HashMap<Uuid, BetaBelief> {
        if is_home {
            self.home.clone()
        } else {
            self.away.clone()
        }
    }
}
