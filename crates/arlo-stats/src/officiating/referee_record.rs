use crate::aggregator::StatAggregator;
use crate::snapshot::{IntoSnapshot, RefereeMatchSnapshot};
use arlo_events::MatchEvent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RefereeMatchStats {
    pub calls_made: u32,
    pub calls_correct: u32,
    pub calls_incorrect: u32,
    pub peace_referee_interventions: u32,
}

impl RefereeMatchStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn calls_made(&self) -> u32 {
        self.calls_made
    }

    pub fn calls_correct(&self) -> u32 {
        self.calls_correct
    }

    pub fn calls_incorrect(&self) -> u32 {
        self.calls_incorrect
    }

    pub fn peace_referee_interventions(&self) -> u32 {
        self.peace_referee_interventions
    }

    pub fn accuracy_rate(&self) -> f64 {
        if self.calls_made == 0 {
            0.0
        } else {
            (self.calls_correct as f64) / (self.calls_made as f64)
        }
    }
}

impl IntoSnapshot for RefereeMatchStats {
    type Snapshot = RefereeMatchSnapshot;

    fn into_snapshot(&self) -> Self::Snapshot {
        RefereeMatchSnapshot {
            calls_made: self.calls_made,
            calls_correct: self.calls_correct,
            calls_incorrect: self.calls_incorrect,
            peace_referee_interventions: self.peace_referee_interventions,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct RefereeStatsAggregator {
    stats: RefereeMatchStats,
}

impl RefereeStatsAggregator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn stats(&self) -> RefereeMatchStats {
        self.stats
    }

    pub fn record_call(&mut self, final_correct: bool, peace_intervened: bool) {
        self.stats.calls_made += 1;
        if final_correct {
            self.stats.calls_correct += 1;
        } else {
            self.stats.calls_incorrect += 1;
        }
        if peace_intervened {
            self.stats.peace_referee_interventions += 1;
        }
    }
}

impl IntoSnapshot for RefereeStatsAggregator {
    type Snapshot = RefereeMatchSnapshot;

    fn into_snapshot(&self) -> Self::Snapshot {
        self.stats.into_snapshot()
    }
}

impl StatAggregator for RefereeStatsAggregator {
    fn handle_event(&mut self, event: &MatchEvent) {
        if let MatchEvent::FoulRaised(e) = event {
            self.record_call(e.final_call_correct(), e.peace_referee_intervened());
        }
    }

    fn reset(&mut self) {
        self.stats = RefereeMatchStats::default();
    }
}