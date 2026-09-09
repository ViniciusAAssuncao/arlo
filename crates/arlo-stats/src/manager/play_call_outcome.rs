use crate::aggregator::StatAggregator;
use arlo_events::MatchEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PlayCallOutcomeStats {
    pub play_call_id: Uuid,
    pub attempts: u32,
    pub successes: u32,
}

impl PlayCallOutcomeStats {
    pub fn new(play_call_id: Uuid) -> Self {
        Self {
            play_call_id,
            attempts: 0,
            successes: 0,
        }
    }

    pub fn play_call_id(&self) -> Uuid {
        self.play_call_id
    }

    pub fn attempts(&self) -> u32 {
        self.attempts
    }

    pub fn successes(&self) -> u32 {
        self.successes
    }

    pub fn failures(&self) -> u32 {
        self.attempts.saturating_sub(self.successes)
    }

    pub fn success_rate(&self) -> f64 {
        if self.attempts == 0 {
            0.0
        } else {
            (self.successes as f64) / (self.attempts as f64)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct PendingPlayCallOutcome {
    play_call_id: Uuid,
    turnover: bool,
    points_scored: u32,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayCallOutcomeAggregator {
    stats: HashMap<Uuid, PlayCallOutcomeStats>,
    pending_play_call: Option<PendingPlayCallOutcome>,
}

impl PlayCallOutcomeAggregator {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
            pending_play_call: None,
        }
    }

    pub fn get(&self, play_call_id: &Uuid) -> Option<&PlayCallOutcomeStats> {
        self.stats.get(play_call_id)
    }

    pub fn get_or_default(&self, play_call_id: &Uuid) -> PlayCallOutcomeStats {
        self.stats
            .get(play_call_id)
            .copied()
            .unwrap_or_else(|| PlayCallOutcomeStats::new(*play_call_id))
    }

    pub fn all_stats(&self) -> &HashMap<Uuid, PlayCallOutcomeStats> {
        &self.stats
    }

    pub fn record_outcome(&mut self, play_call_id: Uuid, success: bool) {
        let entry = self
            .stats
            .entry(play_call_id)
            .or_insert_with(|| PlayCallOutcomeStats::new(play_call_id));
        entry.attempts += 1;
        if success {
            entry.successes += 1;
        }
    }

    fn finalize_pending(&mut self) {
        if let Some(pending) = self.pending_play_call.take() {
            let successful = !pending.turnover && pending.points_scored > 0;
            self.record_outcome(pending.play_call_id, successful);
        }
    }
}

impl StatAggregator for PlayCallOutcomeAggregator {
    fn handle_event(&mut self, event: &MatchEvent) {
        match event {
            MatchEvent::PlayCallSelected(e) => {
                self.finalize_pending();
                self.pending_play_call = Some(PendingPlayCallOutcome {
                    play_call_id: e.play_call_id(),
                    turnover: false,
                    points_scored: 0,
                });
            }
            MatchEvent::GoalPoint(e) => {
                if let Some(pending) = &mut self.pending_play_call {
                    pending.points_scored += e.points();
                }
            }
            MatchEvent::FieldPoint(e) => {
                if let Some(pending) = &mut self.pending_play_call {
                    pending.points_scored += e.points();
                }
            }
            MatchEvent::FieldGoal(e) => {
                if let Some(pending) = &mut self.pending_play_call {
                    pending.points_scored += e.points();
                }
            }
            MatchEvent::Turnover(_) => {
                if let Some(pending) = &mut self.pending_play_call {
                    pending.turnover = true;
                }
            }
            MatchEvent::DownAdvanced(e) => {
                if let Some(pending) = self.pending_play_call.take() {
                    let mirins = e.mirins_advanced_this_down();
                    let successful = !pending.turnover
                        && (pending.points_scored > 0
                            || mirins > 0.0
                            || e.first_down_achieved());
                    self.record_outcome(pending.play_call_id, successful);
                }
            }
            _ => {}
        }
    }

    fn reset(&mut self) {
        self.stats.clear();
        self.pending_play_call = None;
    }
}