use crate::aggregator::StatAggregator;
use crate::snapshot::{IntoSnapshot, PlayerPassingSnapshot};
use arlo_events::MatchEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlayerPassingStats {
    pub player_id: Uuid,
    pub passes_attempted: u32,
    pub passes_completed: u32,
    pub passes_incompleted: u32,
    pub passing_mirins: f64,
    pub longest_pass_mirim: f64,
    pub aerial_passes_attempted: u32,
    pub aerial_passes_completed: u32,
}

impl PlayerPassingStats {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            passes_attempted: 0,
            passes_completed: 0,
            passes_incompleted: 0,
            passing_mirins: 0.0,
            longest_pass_mirim: 0.0,
            aerial_passes_attempted: 0,
            aerial_passes_completed: 0,
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn passes_attempted(&self) -> u32 {
        self.passes_attempted
    }

    pub fn passes_completed(&self) -> u32 {
        self.passes_completed
    }

    pub fn passes_incompleted(&self) -> u32 {
        self.passes_incompleted
    }

    pub fn passing_mirins(&self) -> f64 {
        self.passing_mirins
    }

    pub fn longest_pass_mirim(&self) -> f64 {
        self.longest_pass_mirim
    }

    pub fn aerial_passes_attempted(&self) -> u32 {
        self.aerial_passes_attempted
    }

    pub fn aerial_passes_completed(&self) -> u32 {
        self.aerial_passes_completed
    }

    pub fn completion_rate(&self) -> f64 {
        if self.passes_attempted == 0 {
            0.0
        } else {
            (self.passes_completed as f64) / (self.passes_attempted as f64)
        }
    }

    pub fn aerial_completion_rate(&self) -> f64 {
        if self.aerial_passes_attempted == 0 {
            0.0
        } else {
            (self.aerial_passes_completed as f64) / (self.aerial_passes_attempted as f64)
        }
    }

    pub fn average_mirins_per_completion(&self) -> f64 {
        if self.passes_completed == 0 {
            0.0
        } else {
            self.passing_mirins / (self.passes_completed as f64)
        }
    }

    pub fn average_mirins_per_attempt(&self) -> f64 {
        if self.passes_attempted == 0 {
            0.0
        } else {
            self.passing_mirins / (self.passes_attempted as f64)
        }
    }

    pub fn record_completion(&mut self, distance_mirim: f64, is_aerial: bool) {
        self.passes_attempted += 1;
        self.passes_completed += 1;
        self.passing_mirins += distance_mirim.max(0.0);
        if distance_mirim > self.longest_pass_mirim {
            self.longest_pass_mirim = distance_mirim;
        }
        if is_aerial {
            self.aerial_passes_attempted += 1;
            self.aerial_passes_completed += 1;
        }
    }

    pub fn record_incompletion(&mut self, is_aerial: bool) {
        self.passes_attempted += 1;
        self.passes_incompleted += 1;
        if is_aerial {
            self.aerial_passes_attempted += 1;
        }
    }
}

impl IntoSnapshot for PlayerPassingStats {
    type Snapshot = PlayerPassingSnapshot;

    fn into_snapshot(&self) -> Self::Snapshot {
        PlayerPassingSnapshot {
            player_id: self.player_id,
            passes_attempted: self.passes_attempted,
            passes_completed: self.passes_completed,
            passes_incompleted: self.passes_incompleted,
            passing_mirins: self.passing_mirins,
            longest_pass_mirim: self.longest_pass_mirim,
            aerial_passes_attempted: self.aerial_passes_attempted,
            aerial_passes_completed: self.aerial_passes_completed,
            completion_rate: self.completion_rate(),
            aerial_completion_rate: self.aerial_completion_rate(),
            average_mirins_per_completion: self.average_mirins_per_completion(),
            average_mirins_per_attempt: self.average_mirins_per_attempt(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerPassingAggregator {
    stats: HashMap<Uuid, PlayerPassingStats>,
}

impl PlayerPassingAggregator {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    pub fn get(&self, player_id: &Uuid) -> Option<&PlayerPassingStats> {
        self.stats.get(player_id)
    }

    pub fn get_or_default(&self, player_id: &Uuid) -> PlayerPassingStats {
        self.stats
            .get(player_id)
            .copied()
            .unwrap_or_else(|| PlayerPassingStats::new(*player_id))
    }

    pub fn all_stats(&self) -> &HashMap<Uuid, PlayerPassingStats> {
        &self.stats
    }

    fn get_mut_or_create(&mut self, player_id: Uuid) -> &mut PlayerPassingStats {
        self.stats
            .entry(player_id)
            .or_insert_with(|| PlayerPassingStats::new(player_id))
    }

    pub fn record_completion(&mut self, player_id: Uuid, distance_mirim: f64, is_aerial: bool) {
        self.get_mut_or_create(player_id)
            .record_completion(distance_mirim, is_aerial);
    }

    pub fn record_incompletion(&mut self, player_id: Uuid, is_aerial: bool) {
        self.get_mut_or_create(player_id)
            .record_incompletion(is_aerial);
    }
}

impl IntoSnapshot for PlayerPassingAggregator {
    type Snapshot = HashMap<Uuid, PlayerPassingSnapshot>;

    fn into_snapshot(&self) -> Self::Snapshot {
        self.stats
            .iter()
            .map(|(&id, stats)| (id, stats.into_snapshot()))
            .collect()
    }
}

impl StatAggregator for PlayerPassingAggregator {
    fn handle_event(&mut self, event: &MatchEvent) {
        match event {
            MatchEvent::PassCompleted(e) => {
                self.record_completion(e.passer_id(), e.distance_mirim(), e.is_aerial());
            }
            MatchEvent::ReceptionResolved(e) => {
                if !e.caught() {
                    self.record_incompletion(e.passer_id(), e.is_aerial());
                }
            }
            MatchEvent::DistributionCompleted(e) => {
                if e.caught() {
                    self.record_completion(e.passer_id(), e.distance_mirim(), e.is_aerial());
                } else {
                    self.record_incompletion(e.passer_id(), e.is_aerial());
                }
            }
            _ => {}
        }
    }

    fn reset(&mut self) {
        self.stats.clear();
    }
}