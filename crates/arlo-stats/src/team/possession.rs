use crate::aggregator::StatAggregator;
use crate::snapshot::{IntoSnapshot, TeamPossessionSnapshot};
use arlo_events::MatchEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamPossessionStats {
    pub team_id: Uuid,
    pub total_possession_seconds: f64,
}

impl TeamPossessionStats {
    pub fn new(team_id: Uuid) -> Self {
        Self {
            team_id,
            total_possession_seconds: 0.0,
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn total_possession_seconds(&self) -> f64 {
        self.total_possession_seconds
    }
}

impl IntoSnapshot for TeamPossessionStats {
    type Snapshot = TeamPossessionSnapshot;

    fn into_snapshot(&self) -> Self::Snapshot {
        TeamPossessionSnapshot {
            team_id: self.team_id,
            total_possession_seconds: self.total_possession_seconds,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TeamPossessionAggregator {
    stats: HashMap<Uuid, TeamPossessionStats>,
}

impl TeamPossessionAggregator {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    pub fn get(&self, team_id: &Uuid) -> Option<&TeamPossessionStats> {
        self.stats.get(team_id)
    }

    pub fn get_or_default(&self, team_id: &Uuid) -> TeamPossessionStats {
        self.stats
            .get(team_id)
            .cloned()
            .unwrap_or_else(|| TeamPossessionStats::new(*team_id))
    }

    pub fn all_stats(&self) -> &HashMap<Uuid, TeamPossessionStats> {
        &self.stats
    }

    fn get_mut_or_create(&mut self, team_id: Uuid) -> &mut TeamPossessionStats {
        self.stats
            .entry(team_id)
            .or_insert_with(|| TeamPossessionStats::new(team_id))
    }

    pub fn record_possession_time(&mut self, team_id: Uuid, duration_seconds: f64) {
        let stats = self.get_mut_or_create(team_id);
        stats.total_possession_seconds += duration_seconds.max(0.0);
    }
}

impl IntoSnapshot for TeamPossessionAggregator {
    type Snapshot = HashMap<Uuid, TeamPossessionSnapshot>;

    fn into_snapshot(&self) -> Self::Snapshot {
        self.stats
            .iter()
            .map(|(&id, stats)| (id, stats.into_snapshot()))
            .collect()
    }
}

impl StatAggregator for TeamPossessionAggregator {
    fn handle_event(&mut self, event: &MatchEvent) {
        if let MatchEvent::PossessionTimeRecorded(e) = event {
            self.record_possession_time(e.team_id(), e.live_duration_seconds());
        }
    }

    fn reset(&mut self) {
        self.stats.clear();
    }
}