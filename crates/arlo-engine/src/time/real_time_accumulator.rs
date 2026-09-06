use arlo_math::units::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RealTimeAccumulator {
    accumulated: Duration,
}

impl RealTimeAccumulator {
    pub fn new() -> Self {
        Self {
            accumulated: Duration::new(0.0),
        }
    }

    pub fn add(&mut self, duration: Duration) {
        self.accumulated = Duration::new(self.accumulated.value() + duration.value());
    }

    pub fn total(&self) -> Duration {
        self.accumulated
    }
}

impl Default for RealTimeAccumulator {
    fn default() -> Self {
        Self::new()
    }
}
