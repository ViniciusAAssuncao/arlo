use crate::aggregator::StatAggregator;
use crate::snapshot::{IntoSnapshot, PlayerGoalguardSnapshot};
use arlo_domain::PitchZone;
use arlo_events::MatchEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerGoalguardStats {
    pub player_id: Uuid,
    pub total_recoveries: u32,
    pub first_zone_recoveries: u32,
    pub second_zone_recoveries: u32,
    pub open_field_recoveries: u32,
    pub used_hands_recoveries: u32,
    pub recoveries_by_zone: HashMap<PitchZone, u32>,
}

impl PlayerGoalguardStats {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            total_recoveries: 0,
            first_zone_recoveries: 0,
            second_zone_recoveries: 0,
            open_field_recoveries: 0,
            used_hands_recoveries: 0,
            recoveries_by_zone: HashMap::new(),
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn total_recoveries(&self) -> u32 {
        self.total_recoveries
    }

    pub fn first_zone_recoveries(&self) -> u32 {
        self.first_zone_recoveries
    }

    pub fn second_zone_recoveries(&self) -> u32 {
        self.second_zone_recoveries
    }

    pub fn open_field_recoveries(&self) -> u32 {
        self.open_field_recoveries
    }

    pub fn used_hands_recoveries(&self) -> u32 {
        self.used_hands_recoveries
    }

    pub fn recoveries_by_zone(&self) -> &HashMap<PitchZone, u32> {
        &self.recoveries_by_zone
    }

    pub fn recoveries_for_zone(&self, zone: PitchZone) -> u32 {
        self.recoveries_by_zone.get(&zone).copied().unwrap_or(0)
    }

    pub fn record_recovery(&mut self, zone: PitchZone, used_hands: bool) {
        self.total_recoveries += 1;
        match zone {
            PitchZone::FirstZone => self.first_zone_recoveries += 1,
            PitchZone::SecondZone => self.second_zone_recoveries += 1,
            PitchZone::OpenField => self.open_field_recoveries += 1,
        }
        *self.recoveries_by_zone.entry(zone).or_insert(0) += 1;
        if used_hands {
            self.used_hands_recoveries += 1;
        }
    }
}

impl IntoSnapshot for PlayerGoalguardStats {
    type Snapshot = PlayerGoalguardSnapshot;

    fn into_snapshot(&self) -> Self::Snapshot {
        PlayerGoalguardSnapshot {
            player_id: self.player_id,
            total_recoveries: self.total_recoveries,
            first_zone_recoveries: self.first_zone_recoveries,
            second_zone_recoveries: self.second_zone_recoveries,
            open_field_recoveries: self.open_field_recoveries,
            used_hands_recoveries: self.used_hands_recoveries,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerGoalguardAggregator {
    stats: HashMap<Uuid, PlayerGoalguardStats>,
}

impl PlayerGoalguardAggregator {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    pub fn get(&self, player_id: &Uuid) -> Option<&PlayerGoalguardStats> {
        self.stats.get(player_id)
    }

    pub fn get_or_default(&self, player_id: &Uuid) -> PlayerGoalguardStats {
        self.stats
            .get(player_id)
            .cloned()
            .unwrap_or_else(|| PlayerGoalguardStats::new(*player_id))
    }

    pub fn all_stats(&self) -> &HashMap<Uuid, PlayerGoalguardStats> {
        &self.stats
    }

    fn get_mut_or_create(&mut self, player_id: Uuid) -> &mut PlayerGoalguardStats {
        self.stats
            .entry(player_id)
            .or_insert_with(|| PlayerGoalguardStats::new(player_id))
    }

    pub fn record_recovery(&mut self, player_id: Uuid, zone: PitchZone, used_hands: bool) {
        self.get_mut_or_create(player_id)
            .record_recovery(zone, used_hands);
    }
}

impl IntoSnapshot for PlayerGoalguardAggregator {
    type Snapshot = HashMap<Uuid, PlayerGoalguardSnapshot>;

    fn into_snapshot(&self) -> Self::Snapshot {
        self.stats
            .iter()
            .map(|(&id, stats)| (id, stats.into_snapshot()))
            .collect()
    }
}

impl StatAggregator for PlayerGoalguardAggregator {
    fn handle_event(&mut self, event: &MatchEvent) {
        if let MatchEvent::GoalguardRecoveryResolved(e) = event {
            self.record_recovery(e.goalguard_id(), e.zone(), e.used_hands());
        }
    }

    fn reset(&mut self) {
        self.stats.clear();
    }
}