use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct FatigueState {
    pub cumulative_distance_mirim: f64,
    pub cumulative_high_intensity_actions: u32,
}

impl FatigueState {
    pub fn new(cumulative_distance_mirim: f64, cumulative_high_intensity_actions: u32) -> Self {
        Self {
            cumulative_distance_mirim,
            cumulative_high_intensity_actions,
        }
    }

    pub fn cumulative_distance_mirim(&self) -> f64 {
        self.cumulative_distance_mirim
    }

    pub fn cumulative_high_intensity_actions(&self) -> u32 {
        self.cumulative_high_intensity_actions
    }

    pub fn add_distance(&mut self, mirim: f64) {
        self.cumulative_distance_mirim += mirim;
    }

    pub fn increment_high_intensity_actions(&mut self) {
        self.cumulative_high_intensity_actions += 1;
    }
}