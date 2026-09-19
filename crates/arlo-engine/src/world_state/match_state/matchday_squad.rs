use crate::lineup_runtime::Lineup;
use arlo_domain::Player;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchdaySquad {
    bench: Vec<Arc<Player>>,
}

impl MatchdaySquad {
    pub fn new(bench: Vec<Arc<Player>>) -> Self {
        Self { bench }
    }

    pub fn from_roster_and_lineup(roster: &[Player], lineup: &Lineup) -> Self {
        let lineup_ids: HashSet<Uuid> = lineup.players().iter().map(|p| p.id()).collect();
        let bench = roster
            .iter()
            .filter(|p| !lineup_ids.contains(&p.id()))
            .cloned()
            .map(Arc::new)
            .collect();
        Self { bench }
    }

    pub fn bench(&self) -> &[Arc<Player>] {
        &self.bench
    }

    pub fn bench_mut(&mut self) -> &mut Vec<Arc<Player>> {
        &mut self.bench
    }

    pub fn available_replacements(&self) -> impl Iterator<Item = &Arc<Player>> {
        self.bench.iter()
    }

    pub fn swap_bench(&mut self, outgoing: Arc<Player>, incoming_id: Uuid) {
        self.bench.retain(|p| p.id() != incoming_id);
        if !self.bench.iter().any(|p| p.id() == outgoing.id()) {
            self.bench.push(outgoing);
        }
    }
}