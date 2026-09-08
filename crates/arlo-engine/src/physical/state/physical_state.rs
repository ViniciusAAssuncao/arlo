use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PhysicalState {
    energy: f64,
    w_prime_balance: f64,
    cumulative_distance_mirim: f64,
    cumulative_high_intensity_actions: u32,
}

impl PhysicalState {
    pub fn new(
        energy: f64,
        w_prime_balance: f64,
        cumulative_distance_mirim: f64,
        cumulative_high_intensity_actions: u32,
    ) -> Self {
        Self {
            energy: energy.clamp(0.0, 1.0),
            w_prime_balance: w_prime_balance.clamp(0.0, 1.0),
            cumulative_distance_mirim: cumulative_distance_mirim.max(0.0),
            cumulative_high_intensity_actions,
        }
    }

    pub fn with_energy(energy: f64) -> Self {
        Self {
            energy: energy.clamp(0.0, 1.0),
            w_prime_balance: 1.0,
            cumulative_distance_mirim: 0.0,
            cumulative_high_intensity_actions: 0,
        }
    }

    pub fn initial() -> Self {
        Self::with_energy(1.0)
    }

    pub fn energy(&self) -> f64 {
        self.energy
    }

    pub fn w_prime_balance(&self) -> f64 {
        self.w_prime_balance
    }

    pub fn cumulative_distance_mirim(&self) -> f64 {
        self.cumulative_distance_mirim
    }

    pub fn total_distance_mirim(&self) -> f64 {
        self.cumulative_distance_mirim
    }

    pub fn cumulative_high_intensity_actions(&self) -> u32 {
        self.cumulative_high_intensity_actions
    }

    pub fn set_energy(&mut self, energy: f64) {
        self.energy = energy.clamp(0.0, 1.0);
    }

    pub fn set_w_prime_balance(&mut self, balance: f64) {
        self.w_prime_balance = balance.clamp(0.0, 1.0);
    }

    pub fn add_distance(&mut self, mirim: f64) {
        self.cumulative_distance_mirim += mirim.max(0.0);
    }

    pub fn increment_high_intensity_actions(&mut self) {
        self.cumulative_high_intensity_actions += 1;
    }
}

impl Default for PhysicalState {
    fn default() -> Self {
        Self::initial()
    }
}

pub type FatigueState = PhysicalState;
