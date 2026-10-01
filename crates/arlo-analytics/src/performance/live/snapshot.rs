use crate::performance::rating::PlayerPerformanceSnapshot;
use arlo_events::MatchClockInstant;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LivePerformanceSnapshotRecord {
    sequence_number: u64,
    clock: MatchClockInstant,
    player_snapshots: HashMap<Uuid, PlayerPerformanceSnapshot>,
    team_average_ratings: HashMap<Uuid, f64>,
}

impl LivePerformanceSnapshotRecord {
    pub fn new(
        sequence_number: u64,
        clock: MatchClockInstant,
        player_snapshots: HashMap<Uuid, PlayerPerformanceSnapshot>,
        team_average_ratings: HashMap<Uuid, f64>,
    ) -> Self {
        Self {
            sequence_number,
            clock,
            player_snapshots,
            team_average_ratings,
        }
    }

    pub fn sequence_number(&self) -> u64 {
        self.sequence_number
    }

    pub fn clock(&self) -> MatchClockInstant {
        self.clock
    }

    pub fn player_snapshots(&self) -> &HashMap<Uuid, PlayerPerformanceSnapshot> {
        &self.player_snapshots
    }

    pub fn team_average_ratings(&self) -> &HashMap<Uuid, f64> {
        &self.team_average_ratings
    }

    pub fn player_snapshot(&self, player_id: &Uuid) -> Option<&PlayerPerformanceSnapshot> {
        self.player_snapshots.get(player_id)
    }

    pub fn team_average_rating(&self, team_id: &Uuid) -> Option<f64> {
        self.team_average_ratings.get(team_id).copied()
    }

    pub fn player_snapshots_vec(&self) -> Vec<PlayerPerformanceSnapshot> {
        let mut list: Vec<PlayerPerformanceSnapshot> =
            self.player_snapshots.values().cloned().collect();
        list.sort_by_key(|s| s.player_id());
        list
    }
}