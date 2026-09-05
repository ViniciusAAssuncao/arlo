use crate::time::duration_component::DurationComponentKind;
use arlo_math::units::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DurationLedger {
    live: Vec<(DurationComponentKind, Duration)>,
    dead_ball: Vec<(DurationComponentKind, Duration)>,
}

impl DurationLedger {
    pub fn new() -> Self {
        Self {
            live: Vec::new(),
            dead_ball: Vec::new(),
        }
    }

    pub fn record_live(&mut self, kind: DurationComponentKind, duration: Duration) {
        self.live.push((kind, duration));
    }

    pub fn record_dead_ball(&mut self, kind: DurationComponentKind, duration: Duration) {
        self.dead_ball.push((kind, duration));
    }

    pub fn total_live(&self) -> Duration {
        let total_secs: f64 = self.live.iter().map(|(_, d)| d.value()).sum();
        Duration::new(total_secs)
    }

    pub fn total_dead_ball(&self) -> Duration {
        let total_secs: f64 = self.dead_ball.iter().map(|(_, d)| d.value()).sum();
        Duration::new(total_secs)
    }

    pub fn total(&self) -> Duration {
        Duration::new(self.total_live().value() + self.total_dead_ball().value())
    }

    pub fn merge(&mut self, other: DurationLedger) {
        self.live.extend(other.live);
        self.dead_ball.extend(other.dead_ball);
    }

    pub fn components(&self) -> impl Iterator<Item = &(DurationComponentKind, Duration)> {
        self.live.iter().chain(self.dead_ball.iter())
    }

    pub fn live(&self) -> &[(DurationComponentKind, Duration)] {
        &self.live
    }

    pub fn dead_ball(&self) -> &[(DurationComponentKind, Duration)] {
        &self.dead_ball
    }
}
