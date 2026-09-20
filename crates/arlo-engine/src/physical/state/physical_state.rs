use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PhysicalState {
    energy: f64,
    w_prime_balance: f64,
}

impl PhysicalState {
    pub fn new(energy: f64, w_prime_balance: f64) -> Self {
        Self {
            energy: energy.clamp(0.0, 1.0),
            w_prime_balance: w_prime_balance.clamp(0.0, 1.0),
        }
    }

    pub fn with_energy(energy: f64) -> Self {
        Self {
            energy: energy.clamp(0.0, 1.0),
            w_prime_balance: 1.0,
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

    pub fn intensity_reserve(&self) -> f64 {
        self.w_prime_balance
    }

    pub fn set_energy(&mut self, energy: f64) {
        self.energy = energy.clamp(0.0, 1.0);
    }

    pub fn set_w_prime_balance(&mut self, balance: f64) {
        self.w_prime_balance = balance.clamp(0.0, 1.0);
    }

    pub fn set_intensity_reserve(&mut self, reserve: f64) {
        self.w_prime_balance = reserve.clamp(0.0, 1.0);
    }
}

impl Default for PhysicalState {
    fn default() -> Self {
        Self::initial()
    }
}

pub type FatigueState = PhysicalState;