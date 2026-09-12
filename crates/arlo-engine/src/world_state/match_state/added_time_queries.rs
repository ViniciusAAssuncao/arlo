use crate::officiating::{AddedTimeTracker, StoppageEventKind};
use crate::world_state::match_state::state::MatchState;

impl MatchState {
    pub fn record_stoppage_event(&mut self, kind: StoppageEventKind) {
        self.added_time.record_event(kind);
    }

    pub fn record_period_dead_ball_seconds(&mut self, seconds: f64) {
        self.added_time.add_dead_ball_seconds(seconds);
    }

    pub fn added_time_tracker(&self) -> &AddedTimeTracker {
        &self.added_time
    }

    pub fn mark_added_time_awarded(&mut self, seconds: f64) {
        self.added_time.set_awarded_seconds(seconds);
    }

    pub fn reset_added_time_tracker_for_new_period(&mut self) {
        self.added_time.reset_for_new_period();
    }
}
