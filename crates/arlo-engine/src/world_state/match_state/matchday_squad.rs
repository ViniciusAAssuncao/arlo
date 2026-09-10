use crate::lineup_runtime::Lineup;
use arlo_domain::Player;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchdaySquad {
    bench: Vec<Arc<Player>>,
    substituted_off: HashSet<Uuid>,
}

impl MatchdaySquad {
    pub fn new(bench: Vec<Arc<Player>>, substituted_off: HashSet<Uuid>) -> Self {
        Self {
            bench,
            substituted_off,
        }
    }

    pub fn from_roster_and_lineup(roster: &[Player], lineup: &Lineup) -> Self {
        let lineup_ids: HashSet<Uuid> = lineup.players().iter().map(|p| p.id()).collect();
        let bench = roster
            .iter()
            .filter(|p| !lineup_ids.contains(&p.id()))
            .cloned()
            .map(Arc::new)
            .collect();
        Self {
            bench,
            substituted_off: HashSet::new(),
        }
    }

    pub fn bench(&self) -> &[Arc<Player>] {
        &self.bench
    }

    pub fn bench_mut(&mut self) -> &mut Vec<Arc<Player>> {
        &mut self.bench
    }

    pub fn substituted_off(&self) -> &HashSet<Uuid> {
        &self.substituted_off
    }

    pub fn available_replacements(&self) -> impl Iterator<Item = &Arc<Player>> {
        self.bench
            .iter()
            .filter(move |p| !self.substituted_off.contains(&p.id()))
    }

    pub fn mark_substituted(&mut self, player_id: Uuid) {
        self.substituted_off.insert(player_id);
    }

    pub fn clear_substituted_off(&mut self) {
        self.substituted_off.clear();
    }

    pub fn swap_bench(&mut self, outgoing: Arc<Player>, incoming_id: Uuid) {
        self.bench.retain(|p| p.id() != incoming_id);
        if !self.bench.iter().any(|p| p.id() == outgoing.id()) {
            self.bench.push(outgoing);
        }
    }
}