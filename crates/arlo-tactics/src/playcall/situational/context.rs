use arlo_domain::sport_constants::{
    GOAL_POINT_REQUIRED_DRIVES, MAX_CALL_TO_ACTIONS_PER_SERIES, MINIMUM_ADVANCE_MIRINS_PER_SERIES,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct SituationalContext {
    down_pressure: f64,
    distance_urgency: f64,
    scoring_proximity: f64,
    drive_scarcity: f64,
}

impl SituationalContext {
    pub fn new(
        down_pressure: f64,
        distance_urgency: f64,
        scoring_proximity: f64,
        drive_scarcity: f64,
    ) -> Self {
        Self {
            down_pressure,
            distance_urgency,
            scoring_proximity,
            drive_scarcity,
        }
    }

    pub fn down_pressure(&self) -> f64 {
        self.down_pressure
    }

    pub fn distance_urgency(&self) -> f64 {
        self.distance_urgency
    }

    pub fn scoring_proximity(&self) -> f64 {
        self.scoring_proximity
    }

    pub fn drive_scarcity(&self) -> f64 {
        self.drive_scarcity
    }
}

pub fn derive_down_pressure(down: u8) -> f64 {
    let clamped = down.clamp(1, MAX_CALL_TO_ACTIONS_PER_SERIES as u8);
    (clamped - 1) as f64 / (MAX_CALL_TO_ACTIONS_PER_SERIES - 1) as f64
}

pub fn derive_distance_urgency(remaining_advance_mirim: f64) -> f64 {
    (remaining_advance_mirim / MINIMUM_ADVANCE_MIRINS_PER_SERIES).clamp(0.0, 1.0)
}

pub fn derive_drive_scarcity(drives_in_series: u32) -> f64 {
    1.0 - (drives_in_series as f64 / GOAL_POINT_REQUIRED_DRIVES as f64).clamp(0.0, 1.0)
}

pub fn derive_scoring_proximity(normalized_x_to_goal: f64) -> f64 {
    normalized_x_to_goal.clamp(0.0, 1.0)
}