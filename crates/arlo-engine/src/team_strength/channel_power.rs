use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct TeamMatchPower {
    offensive_power: f64,
    defensive_power: f64,
    control_power: f64,
}

impl TeamMatchPower {
    pub fn new(offensive_power: f64, defensive_power: f64, control_power: f64) -> Self {
        Self {
            offensive_power,
            defensive_power,
            control_power,
        }
    }

    pub fn offensive_power(&self) -> f64 {
        self.offensive_power
    }

    pub fn defensive_power(&self) -> f64 {
        self.defensive_power
    }

    pub fn control_power(&self) -> f64 {
        self.control_power
    }
}
