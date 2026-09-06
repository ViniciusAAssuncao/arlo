use crate::aggregator::StatAggregator;
use arlo_events::{DuelKind, MatchEvent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlayerReceivingStats {
    pub player_id: Uuid,
    pub targets: u32,
    pub receptions: u32,
    pub drops: u32,
    pub run_after_catch_mirins: f64,
    pub longest_reception_mirim: f64,
    pub receiving_mirins: f64,
}

impl PlayerReceivingStats {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            targets: 0,
            receptions: 0,
            drops: 0,
            run_after_catch_mirins: 0.0,
            longest_reception_mirim: 0.0,
            receiving_mirins: 0.0,
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn targets(&self) -> u32 {
        self.targets
    }

    pub fn receptions(&self) -> u32 {
        self.receptions
    }

    pub fn drops(&self) -> u32 {
        self.drops
    }

    pub fn run_after_catch_mirins(&self) -> f64 {
        self.run_after_catch_mirins
    }

    pub fn longest_reception_mirim(&self) -> f64 {
        self.longest_reception_mirim
    }

    pub fn receiving_mirins(&self) -> f64 {
        self.receiving_mirins
    }

    pub fn total_receiving_mirins(&self) -> f64 {
        self.receiving_mirins
    }

    pub fn catch_rate(&self) -> f64 {
        if self.targets == 0 {
            0.0
        } else {
            (self.receptions as f64) / (self.targets as f64)
        }
    }

    pub fn drop_rate(&self) -> f64 {
        if self.targets == 0 {
            0.0
        } else {
            (self.drops as f64) / (self.targets as f64)
        }
    }

    pub fn average_mirins_per_reception(&self) -> f64 {
        if self.receptions == 0 {
            0.0
        } else {
            self.receiving_mirins / (self.receptions as f64)
        }
    }

    pub fn average_rac_per_reception(&self) -> f64 {
        if self.receptions == 0 {
            0.0
        } else {
            self.run_after_catch_mirins / (self.receptions as f64)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct PendingReception {
    receiver_id: Uuid,
    had_rac_duel: bool,
    rac_duel_won: bool,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerReceivingAggregator {
    stats: HashMap<Uuid, PlayerReceivingStats>,
    current_reception: Option<PendingReception>,
}

impl PlayerReceivingAggregator {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
            current_reception: None,
        }
    }

    pub fn get(&self, player_id: &Uuid) -> Option<&PlayerReceivingStats> {
        self.stats.get(player_id)
    }

    pub fn get_or_default(&self, player_id: &Uuid) -> PlayerReceivingStats {
        self.stats
            .get(player_id)
            .copied()
            .unwrap_or_else(|| PlayerReceivingStats::new(*player_id))
    }

    pub fn all_stats(&self) -> &HashMap<Uuid, PlayerReceivingStats> {
        &self.stats
    }

    fn get_mut_or_create(&mut self, player_id: Uuid) -> &mut PlayerReceivingStats {
        self.stats
            .entry(player_id)
            .or_insert_with(|| PlayerReceivingStats::new(player_id))
    }

    pub fn record_target(&mut self, player_id: Uuid) {
        let stats = self.get_mut_or_create(player_id);
        stats.targets += 1;
    }

    pub fn record_reception(&mut self, player_id: Uuid, mirins: f64, rac_mirins: f64) {
        let stats = self.get_mut_or_create(player_id);
        stats.targets += 1;
        stats.receptions += 1;
        stats.receiving_mirins += mirins;
        stats.run_after_catch_mirins += rac_mirins;
        if mirins > stats.longest_reception_mirim {
            stats.longest_reception_mirim = mirins;
        }
    }

    pub fn record_drop(&mut self, player_id: Uuid) {
        let stats = self.get_mut_or_create(player_id);
        stats.targets += 1;
        stats.drops += 1;
    }

    pub fn record_rac(&mut self, player_id: Uuid, rac_mirins: f64) {
        let stats = self.get_mut_or_create(player_id);
        stats.run_after_catch_mirins += rac_mirins;
    }

    fn finalize_pending(&mut self) {
        self.current_reception = None;
    }
}

impl StatAggregator for PlayerReceivingAggregator {
    fn handle_event(&mut self, event: &MatchEvent) {
        match event {
            MatchEvent::CallToActionStarted(_) => {
                self.finalize_pending();
            }
            MatchEvent::PassCompleted(e) => {
                let stats = self.get_mut_or_create(e.receiver_id());
                stats.targets += 1;
                stats.receptions += 1;
                stats.receiving_mirins += e.distance_mirim();
                if e.distance_mirim() > stats.longest_reception_mirim {
                    stats.longest_reception_mirim = e.distance_mirim();
                }
            }
            MatchEvent::DistributionCompleted(e) => {
                self.finalize_pending();
                let stats = self.get_mut_or_create(e.receiver_id());
                stats.targets += 1;
                if e.caught() {
                    stats.receptions += 1;
                    stats.receiving_mirins += e.distance_mirim();
                    if e.distance_mirim() > stats.longest_reception_mirim {
                        stats.longest_reception_mirim = e.distance_mirim();
                    }
                    self.current_reception = Some(PendingReception {
                        receiver_id: e.receiver_id(),
                        had_rac_duel: false,
                        rac_duel_won: false,
                    });
                } else {
                    stats.drops += 1;
                    self.current_reception = None;
                }
            }
            MatchEvent::ReceptionResolved(_) => {
                self.finalize_pending();
            }
            MatchEvent::DuelResolved(e) => {
                if let Some(pending) = &mut self.current_reception {
                    if e.kind() == DuelKind::RunBreakthrough {
                        pending.had_rac_duel = true;
                        if e.attacker_won() {
                            pending.rac_duel_won = true;
                        }
                    }
                }
            }
            MatchEvent::DownAdvanced(e) => {
                if let Some(pending) = self.current_reception.take() {
                    if pending.had_rac_duel && pending.rac_duel_won {
                        let mirins = e.mirins_advanced_this_down();
                        let stats = self.get_mut_or_create(pending.receiver_id);
                        let rac = mirins.max(0.0);
                        stats.run_after_catch_mirins += rac;
                    }
                }
            }
            _ => {}
        }
    }

    fn reset(&mut self) {
        self.stats.clear();
        self.current_reception = None;
    }
}