use crate::aggregator::StatAggregator;
use crate::snapshot::{IntoSnapshot, PlayerAssistSnapshot};
use arlo_events::MatchEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerAssistStats {
    pub player_id: Uuid,
    pub goalpoint_assists: u32,
}

impl PlayerAssistStats {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            goalpoint_assists: 0,
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn goalpoint_assists(&self) -> u32 {
        self.goalpoint_assists
    }
}

impl IntoSnapshot for PlayerAssistStats {
    type Snapshot = PlayerAssistSnapshot;

    fn into_snapshot(&self) -> Self::Snapshot {
        PlayerAssistSnapshot {
            player_id: self.player_id,
            goalpoint_assists: self.goalpoint_assists,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerAssistAggregator {
    stats: HashMap<Uuid, PlayerAssistStats>,
}

impl PlayerAssistAggregator {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    pub fn get(&self, player_id: &Uuid) -> Option<&PlayerAssistStats> {
        self.stats.get(player_id)
    }

    pub fn get_or_default(&self, player_id: &Uuid) -> PlayerAssistStats {
        self.stats
            .get(player_id)
            .copied()
            .unwrap_or_else(|| PlayerAssistStats::new(*player_id))
    }

    pub fn all_stats(&self) -> &HashMap<Uuid, PlayerAssistStats> {
        &self.stats
    }

    fn get_mut_or_create(&mut self, player_id: Uuid) -> &mut PlayerAssistStats {
        self.stats
            .entry(player_id)
            .or_insert_with(|| PlayerAssistStats::new(player_id))
    }

    pub fn record_goalpoint_assist(&mut self, player_id: Uuid) {
        let stats = self.get_mut_or_create(player_id);
        stats.goalpoint_assists += 1;
    }
}

impl IntoSnapshot for PlayerAssistAggregator {
    type Snapshot = HashMap<Uuid, PlayerAssistSnapshot>;

    fn into_snapshot(&self) -> Self::Snapshot {
        self.stats
            .iter()
            .map(|(&id, stats)| (id, stats.into_snapshot()))
            .collect()
    }
}

impl StatAggregator for PlayerAssistAggregator {
    fn handle_event(&mut self, event: &MatchEvent) {
        if let MatchEvent::GoalPoint(e) = event {
            if let Some(assister_id) = e.assister_id() {
                self.record_goalpoint_assist(assister_id);
            }
        }
    }

    fn reset(&mut self) {
        self.stats.clear();
    }
}
