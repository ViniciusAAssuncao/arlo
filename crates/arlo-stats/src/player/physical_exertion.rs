use crate::aggregator::StatAggregator;
use crate::snapshot::{IntoSnapshot, PlayerPhysicalSnapshot};
use arlo_events::{MatchEvent, PitchZone};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerPhysicalStats {
    pub player_id: Uuid,
    pub end_energy_level: f64,
    pub peak_anaerobic_depletion: f64,
    pub total_distance_covered: f64,
    pub high_intensity_distance: f64,
    pub low_intensity_distance: f64,
    pub metabolic_energy_joules: f64,
    pub peak_speed_meters_per_sec: f64,
    pub intra_match_recovery_amount: f64,
    pub distance_first_zone: f64,
    pub distance_second_zone: f64,
    pub distance_corridors: f64,
    pub distance_central: f64,
    pub by_zone: HashMap<PitchZone, f64>,
}

impl PlayerPhysicalStats {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            end_energy_level: 1.0,
            peak_anaerobic_depletion: 0.0,
            total_distance_covered: 0.0,
            high_intensity_distance: 0.0,
            low_intensity_distance: 0.0,
            metabolic_energy_joules: 0.0,
            peak_speed_meters_per_sec: 0.0,
            intra_match_recovery_amount: 0.0,
            distance_first_zone: 0.0,
            distance_second_zone: 0.0,
            distance_corridors: 0.0,
            distance_central: 0.0,
            by_zone: HashMap::new(),
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

    pub fn high_intensity_distance(&self) -> f64 {
        self.high_intensity_distance
    }

    pub fn low_intensity_distance(&self) -> f64 {
        self.low_intensity_distance
    }

    pub fn metabolic_energy_joules(&self) -> f64 {
        self.metabolic_energy_joules
    }

    pub fn peak_speed_meters_per_sec(&self) -> f64 {
        self.peak_speed_meters_per_sec
    }

    pub fn intra_match_recovery_amount(&self) -> f64 {
        self.intra_match_recovery_amount
    }

    pub fn distance_first_zone(&self) -> f64 {
        self.distance_first_zone
    }

    pub fn distance_second_zone(&self) -> f64 {
        self.distance_second_zone
    }

    pub fn distance_corridors(&self) -> f64 {
        self.distance_corridors
    }

    pub fn distance_central(&self) -> f64 {
        self.distance_central
    }

    pub fn by_zone(&self) -> &HashMap<PitchZone, f64> {
        &self.by_zone
    }

    pub fn distance_in_zone(&self, zone: PitchZone) -> f64 {
        self.by_zone.get(&zone).copied().unwrap_or(0.0)
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
            high_intensity_distance: self.high_intensity_distance,
            low_intensity_distance: self.low_intensity_distance,
            metabolic_energy_joules: self.metabolic_energy_joules,
            peak_speed_meters_per_sec: self.peak_speed_meters_per_sec,
            intra_match_recovery_amount: self.intra_match_recovery_amount,
            distance_first_zone: self.distance_first_zone,
            distance_second_zone: self.distance_second_zone,
            distance_corridors: self.distance_corridors,
            distance_central: self.distance_central,
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
            .cloned()
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
        high_intensity_distance_mirim: f64,
        low_intensity_distance_mirim: f64,
        metabolic_energy_joules: f64,
        zone: PitchZone,
        peak_speed: f64,
    ) {
        let stats = self.get_mut_or_create(player_id);
        stats.end_energy_level = energy.clamp(0.0, 1.0);
        let depletion = (1.0 - w_prime_balance).clamp(0.0, 1.0);
        if depletion > stats.peak_anaerobic_depletion {
            stats.peak_anaerobic_depletion = depletion;
        }
        stats.total_distance_covered += distance_mirim.max(0.0);
        stats.high_intensity_distance += high_intensity_distance_mirim.max(0.0);
        stats.low_intensity_distance += low_intensity_distance_mirim.max(0.0);
        stats.metabolic_energy_joules += metabolic_energy_joules.max(0.0);
        if peak_speed > stats.peak_speed_meters_per_sec {
            stats.peak_speed_meters_per_sec = peak_speed;
        }

        match zone {
            PitchZone::FirstZone => stats.distance_first_zone += distance_mirim.max(0.0),
            PitchZone::SecondZone => stats.distance_second_zone += distance_mirim.max(0.0),
            PitchZone::Corridor => stats.distance_corridors += distance_mirim.max(0.0),
            PitchZone::Central => stats.distance_central += distance_mirim.max(0.0),
        }

        *stats.by_zone.entry(zone).or_insert(0.0) += distance_mirim.max(0.0);
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
                    e.high_intensity_distance_mirim(),
                    e.low_intensity_distance_mirim(),
                    e.metabolic_energy_joules(),
                    e.zone(),
                    e.peak_speed_meters_per_sec(),
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