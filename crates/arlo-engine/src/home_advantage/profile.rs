use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HomeAdvantageProfile {
    base_intensity: f64,
    duel_logit_per_intensity: f64,
    impulse_baseline_boost_per_intensity: f64,
    momentum_resilience_per_intensity: f64,
    power_z_boost_per_intensity: f64,
}

impl Default for HomeAdvantageProfile {
    fn default() -> Self {
        Self {
            base_intensity: 1.0,
            duel_logit_per_intensity: 0.20,
            impulse_baseline_boost_per_intensity: 2.0,
            momentum_resilience_per_intensity: 0.10,
            power_z_boost_per_intensity: 0.15,
        }
    }
}

impl HomeAdvantageProfile {
    pub fn duel_logit(&self) -> f64 {
        self.base_intensity * self.duel_logit_per_intensity
    }

    pub fn impulse_baseline_boost(&self) -> f64 {
        self.base_intensity * self.impulse_baseline_boost_per_intensity
    }

    pub fn momentum_resilience(&self) -> f64 {
        self.base_intensity * self.momentum_resilience_per_intensity
    }

    pub fn power_z_boost(&self) -> f64 {
        self.base_intensity * self.power_z_boost_per_intensity
    }
}
