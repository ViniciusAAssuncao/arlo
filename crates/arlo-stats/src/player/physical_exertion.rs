use crate::aggregator::StatAggregator;
use crate::snapshot::{IntoSnapshot, PlayerPhysicalSnapshot};
use arlo_events::MatchEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlayerPhysicalStats {
    pub player_id: Uuid,
    pub end_energy_level: f64,
    pub peak_anaerobic_depletion: f64,
    pub total_distance_covered: f64,
    pub intra_match_recovery_amount: f64,
}

impl PlayerPhysicalStats {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            end_energy_level: 1.0,
            peak_anaerobic_depletion: 0.0,
            total_distance_covered: 0.0,
            intra_match_recovery_amount: 0.0,
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn end_energy_level(&self) -> f64 {
        self.end_energy_level
    }

    pub fn peak_anaerobic_depletion(&self) -> f64 {
        self.peak_anaerobic_depletion
    }

    pub fn total_distance_covered(&self) -> f64 {
        self.total_distance_covered
    }

    pub fn intra_match_recovery_amount(&self) -> f64 {
        self.intra_match_recovery_amount
    }
}

impl IntoSnapshot for PlayerPhysicalStats {
    type Snapshot = PlayerPhysicalSnapshot;

    fn into_snapshot(&self) -> Self::Snapshot {
        PlayerPhysicalSnapshot {
            player_id: self.player_id,
            end_energy_level: self.end_energy_level,
            peak_anaerobic_depletion: self.peak_anaerobic_depletion,
            total_distance_covered: self.total_distance_covered,
            intra_match_recovery_amount: self.intra_match_recovery_amount,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerPhysicalAggregator {
    stats: HashMap<Uuid, PlayerPhysicalStats>,
}

pub type PlayerPhysicalExertionAggregator = PlayerPhysicalAggregator;

impl PlayerPhysicalAggregator {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    pub fn get(&self, player_id: &Uuid) -> Option<&PlayerPhysicalStats> {
        self.stats.get(player_id)
    }

    pub fn get_or_default(&self, player_id: &Uuid) -> PlayerPhysicalStats {
        self.stats
            .get(player_id)
            .copied()
            .unwrap_or_else(|| PlayerPhysicalStats::new(*player_id))
    }

    pub fn all_stats(&self) -> &HashMap<Uuid, PlayerPhysicalStats> {
        &self.stats
    }

    fn get_mut_or_create(&mut self, player_id: Uuid) -> &mut PlayerPhysicalStats {
        self.stats
            .entry(player_id)
            .or_insert_with(|| PlayerPhysicalStats::new(player_id))
    }

    pub fn record_strain(
        &mut self,
        player_id: Uuid,
        energy: f64,
        w_prime_balance: f64,
        distance_mirim: f64,
    ) {
        let stats = self.get_mut_or_create(player_id);
        stats.end_energy_level = energy.clamp(0.0, 1.0);
        let depletion = (1.0 - w_prime_balance).clamp(0.0, 1.0);
        if depletion > stats.peak_anaerobic_depletion {
            stats.peak_anaerobic_depletion = depletion;
        }
        stats.total_distance_covered += distance_mirim.max(0.0);
    }

    pub fn record_recovery(&mut self, player_id: Uuid, recovery_amount: f64) {
        let stats = self.get_mut_or_create(player_id);
        stats.intra_match_recovery_amount += recovery_amount.max(0.0);
    }
}

impl IntoSnapshot for PlayerPhysicalAggregator {
    type Snapshot = HashMap<Uuid, PlayerPhysicalSnapshot>;

    fn into_snapshot(&self) -> Self::Snapshot {
        self.stats
            .iter()
            .map(|(&id, stats)| (id, stats.into_snapshot()))
            .collect()
    }
}

impl StatAggregator for PlayerPhysicalAggregator {
    fn handle_event(&mut self, event: &MatchEvent) {
        match event {
            MatchEvent::PhysicalStrainRecorded(e) => {
                self.record_strain(
                    e.player_id(),
                    e.energy_remaining(),
                    e.w_prime_balance(),
                    e.distance_delta_mirim(),
                );
            }
            MatchEvent::RecoveryIntervalProcessed(e) => {
                self.record_recovery(e.player_id(), e.recovery_amount());
            }
            _ => {}
        }
    }

    fn reset(&mut self) {
        self.stats.clear();
    }
}