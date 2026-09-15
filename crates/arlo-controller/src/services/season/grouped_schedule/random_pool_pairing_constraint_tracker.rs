use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct RandomPoolPairingConstraintTracker {
    used_pairs: HashSet<(Uuid, Uuid)>,
    pair_counts: HashMap<(Uuid, Uuid), u32>,
    pair_last_round: HashMap<(Uuid, Uuid), u32>,
    bye_counts: HashMap<Uuid, u32>,
}

impl RandomPoolPairingConstraintTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn normalize_pair(a: Uuid, b: Uuid) -> (Uuid, Uuid) {
        if a < b {
            (a, b)
        } else {
            (b, a)
        }
    }

    pub fn is_pair_used(&self, a: Uuid, b: Uuid) -> bool {
        self.used_pairs.contains(&Self::normalize_pair(a, b))
    }

    pub fn record_pair(&mut self, a: Uuid, b: Uuid, round_index: u32) {
        let pair = Self::normalize_pair(a, b);
        self.used_pairs.insert(pair);
        *self.pair_counts.entry(pair).or_insert(0) += 1;
        self.pair_last_round.insert(pair, round_index);
    }

    pub fn record_bye(&mut self, team_id: Uuid) {
        *self.bye_counts.entry(team_id).or_insert(0) += 1;
    }

    pub fn bye_count(&self, team_id: &Uuid) -> u32 {
        self.bye_counts.get(team_id).copied().unwrap_or(0)
    }

    pub fn pair_usage_count(&self, a: Uuid, b: Uuid) -> u32 {
        self.pair_counts
            .get(&Self::normalize_pair(a, b))
            .copied()
            .unwrap_or(0)
    }

    pub fn pair_last_round(&self, a: Uuid, b: Uuid) -> Option<u32> {
        self.pair_last_round
            .get(&Self::normalize_pair(a, b))
            .copied()
    }

    pub fn used_pairs(&self) -> &HashSet<(Uuid, Uuid)> {
        &self.used_pairs
    }

    pub fn bye_counts(&self) -> &HashMap<Uuid, u32> {
        &self.bye_counts
    }
}
