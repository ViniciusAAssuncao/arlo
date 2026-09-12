use crate::aggregator::StatAggregator;
use crate::officiating::keyed_registry::{KeyedStat, KeyedStatRegistry};
use crate::snapshot::{IntoSnapshot, PlayerAvailabilitySnapshot};
use arlo_events::{AvailabilityStatus, MatchEvent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerAvailabilityStats {
    pub player_id: Uuid,
    pub total_suspended_seconds: f64,
    pub expulsion_count: u32,
    pub is_currently_expelled: bool,
}

impl PlayerAvailabilityStats {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            total_suspended_seconds: 0.0,
            expulsion_count: 0,
            is_currently_expelled: false,
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn total_suspended_seconds(&self) -> f64 {
        self.total_suspended_seconds
    }

    pub fn expulsion_count(&self) -> u32 {
        self.expulsion_count
    }

    pub fn is_currently_expelled(&self) -> bool {
        self.is_currently_expelled
    }

    pub fn record_transition(
        &mut self,
        _previous: AvailabilityStatus,
        new_status: AvailabilityStatus,
        remaining_seconds: Option<f64>,
    ) {
        match new_status {
            AvailabilityStatus::Suspended => {
                if let Some(secs) = remaining_seconds {
                    self.total_suspended_seconds += secs;
                }
            }
            AvailabilityStatus::Expelled => {
                self.expulsion_count += 1;
                self.is_currently_expelled = true;
            }
            AvailabilityStatus::Active | AvailabilityStatus::Injured => {}
        }
    }
}

impl KeyedStat for PlayerAvailabilityStats {
    fn new_for(player_id: Uuid) -> Self {
        Self::new(player_id)
    }
}

impl IntoSnapshot for PlayerAvailabilityStats {
    type Snapshot = PlayerAvailabilitySnapshot;

    fn into_snapshot(&self) -> Self::Snapshot {
        PlayerAvailabilitySnapshot {
            player_id: self.player_id,
            total_suspended_seconds: self.total_suspended_seconds,
            expulsion_count: self.expulsion_count,
            is_currently_expelled: self.is_currently_expelled,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerAvailabilityAggregator {
    registry: KeyedStatRegistry<PlayerAvailabilityStats>,
}

impl PlayerAvailabilityAggregator {
    pub fn new() -> Self {
        Self {
            registry: KeyedStatRegistry::new(),
        }
    }

    pub fn get(&self, player_id: &Uuid) -> Option<&PlayerAvailabilityStats> {
        self.registry.get(player_id)
    }

    pub fn get_or_default(&self, player_id: &Uuid) -> PlayerAvailabilityStats {
        self.registry.get_or_default(player_id)
    }

    pub fn all_stats(&self) -> &HashMap<Uuid, PlayerAvailabilityStats> {
        self.registry.all_stats()
    }

    pub fn record_change(
        &mut self,
        player_id: Uuid,
        previous: AvailabilityStatus,
        new_status: AvailabilityStatus,
        remaining_seconds: Option<f64>,
    ) {
        let stats = self.registry.entry_or_default(player_id);
        stats.record_transition(previous, new_status, remaining_seconds);
    }
}

impl IntoSnapshot for PlayerAvailabilityAggregator {
    type Snapshot = HashMap<Uuid, PlayerAvailabilitySnapshot>;

    fn into_snapshot(&self) -> Self::Snapshot {
        self.registry
            .all_stats()
            .iter()
            .map(|(&id, stats)| (id, stats.into_snapshot()))
            .collect()
    }
}

impl StatAggregator for PlayerAvailabilityAggregator {
    fn handle_event(&mut self, event: &MatchEvent) {
        if let MatchEvent::PlayerAvailabilityChanged(e) = event {
            self.record_change(
                e.player_id(),
                e.previous_status(),
                e.new_status(),
                e.remaining_seconds(),
            );
        }
    }

    fn reset(&mut self) {
        self.registry.clear();
    }
}