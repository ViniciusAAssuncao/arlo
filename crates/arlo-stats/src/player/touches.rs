use crate::aggregator::StatAggregator;
use arlo_events::MatchEvent;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerTouchStats {
    pub player_id: Uuid,
    pub passes_attempted: u32,
    pub passes_received: u32,
    pub drives_recorded: u32,
    pub recoveries: u32,
    pub scoring_attempts: u32,
    pub total_touches: u32,
}

impl PlayerTouchStats {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            passes_attempted: 0,
            passes_received: 0,
            drives_recorded: 0,
            recoveries: 0,
            scoring_attempts: 0,
            total_touches: 0,
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn passes_attempted(&self) -> u32 {
        self.passes_attempted
    }

    pub fn passes_received(&self) -> u32 {
        self.passes_received
    }

    pub fn drives_recorded(&self) -> u32 {
        self.drives_recorded
    }

    pub fn recoveries(&self) -> u32 {
        self.recoveries
    }

    pub fn scoring_attempts(&self) -> u32 {
        self.scoring_attempts
    }

    pub fn total_touches(&self) -> u32 {
        self.total_touches
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerTouchesAggregator {
    stats: HashMap<Uuid, PlayerTouchStats>,
}

impl PlayerTouchesAggregator {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }

    pub fn get(&self, player_id: &Uuid) -> Option<&PlayerTouchStats> {
        self.stats.get(player_id)
    }

    pub fn get_or_default(&self, player_id: &Uuid) -> PlayerTouchStats {
        self.stats
            .get(player_id)
            .copied()
            .unwrap_or_else(|| PlayerTouchStats::new(*player_id))
    }

    pub fn all_stats(&self) -> &HashMap<Uuid, PlayerTouchStats> {
        &self.stats
    }

    fn get_mut_or_create(&mut self, player_id: Uuid) -> &mut PlayerTouchStats {
        self.stats.entry(player_id).or_insert_with(|| PlayerTouchStats::new(player_id))
    }

    pub fn record_pass_attempt(&mut self, player_id: Uuid) {
        let stats = self.get_mut_or_create(player_id);
        stats.passes_attempted += 1;
        stats.total_touches += 1;
    }

    pub fn record_pass_reception(&mut self, player_id: Uuid) {
        let stats = self.get_mut_or_create(player_id);
        stats.passes_received += 1;
        stats.total_touches += 1;
    }

    pub fn record_drive(&mut self, player_id: Uuid) {
        let stats = self.get_mut_or_create(player_id);
        stats.drives_recorded += 1;
        stats.total_touches += 1;
    }

    pub fn record_recovery(&mut self, player_id: Uuid) {
        let stats = self.get_mut_or_create(player_id);
        stats.recoveries += 1;
        stats.total_touches += 1;
    }

    pub fn record_scoring_attempt(&mut self, player_id: Uuid) {
        let stats = self.get_mut_or_create(player_id);
        stats.scoring_attempts += 1;
        stats.total_touches += 1;
    }
}

impl StatAggregator for PlayerTouchesAggregator {
    fn handle_event(&mut self, event: &MatchEvent) {
        match event {
            MatchEvent::CallToActionStarted(e) => {
                self.record_pass_attempt(e.passer_id());
            }
            MatchEvent::PassCompleted(e) => {
                self.record_pass_reception(e.receiver_id());
            }
            MatchEvent::DriveRecorded(e) => {
                self.record_drive(e.artrine_id());
            }
            MatchEvent::Turnover(e) => {
                if let Some(pid) = e.recovering_player_id() {
                    self.record_recovery(pid);
                }
            }
            MatchEvent::GoalPoint(e) => {
                self.record_scoring_attempt(e.scorer_id());
            }
            MatchEvent::FieldPoint(e) => {
                self.record_scoring_attempt(e.scorer_id());
            }
            MatchEvent::FieldGoal(e) => {
                self.record_scoring_attempt(e.scorer_id());
            }
            _ => {}
        }
    }

    fn reset(&mut self) {
        self.stats.clear();
    }
}
