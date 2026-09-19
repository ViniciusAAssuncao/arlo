use crate::injury::exposure::position_workload_multiplier;
use arlo_domain::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct PlayerExposureRecord {
    live_seconds_on_field: f64,
    weighted_workload_units: f64,
    actions_participated: u32,
}

impl PlayerExposureRecord {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_live_seconds(&mut self, seconds: f64, position: Position) {
        let mult = position_workload_multiplier(position);
        self.live_seconds_on_field += seconds;
        self.weighted_workload_units += seconds * mult;
    }

    pub fn record_action_participation(&mut self) {
        self.actions_participated += 1;
    }

    pub fn live_seconds_on_field(&self) -> f64 {
        self.live_seconds_on_field
    }

    pub fn weighted_workload_units(&self) -> f64 {
        self.weighted_workload_units
    }

    pub fn actions_participated(&self) -> u32 {
        self.actions_participated
    }
}