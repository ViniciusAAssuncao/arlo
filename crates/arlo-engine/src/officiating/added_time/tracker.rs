use crate::officiating::added_time::stoppage_event_kind::StoppageEventKind;
use crate::officiating::added_time::stoppage_log::PeriodStoppageLog;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddedTimeTracker {
    current_period_log: PeriodStoppageLog,
    last_awarded_seconds: f64,
}

impl AddedTimeTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_event(&mut self, kind: StoppageEventKind) {
        self.current_period_log.record(kind);
    }

    pub fn add_dead_ball_seconds(&mut self, seconds: f64) {
        self.current_period_log.add_dead_ball_seconds(seconds);
    }

    pub fn current_log(&self) -> &PeriodStoppageLog {
        &self.current_period_log
    }

    pub fn set_awarded_seconds(&mut self, seconds: f64) {
        self.last_awarded_seconds = seconds;
    }

    pub fn awarded_seconds(&self) -> f64 {
        self.last_awarded_seconds
    }

    pub fn reset_for_new_period(&mut self) {
        self.current_period_log = PeriodStoppageLog::default();
        self.last_awarded_seconds = 0.0;
    }
}
