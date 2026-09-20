use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EnergyTuningProfile {
    base_drain_per_live_second: f64,
    participation_drain_bonus: f64,
    dead_ball_recovery_rate_per_second: f64,
    time_call_recovery_multiplier: f64,
    tempo_drain_scale: f64,
    pressing_drain_scale: f64,
}

impl EnergyTuningProfile {
    pub fn new(
        base_drain_per_live_second: f64,
        participation_drain_bonus: f64,
        dead_ball_recovery_rate_per_second: f64,
        time_call_recovery_multiplier: f64,
        tempo_drain_scale: f64,
        pressing_drain_scale: f64,
    ) -> Self {
        Self {
            base_drain_per_live_second,
            participation_drain_bonus,
            dead_ball_recovery_rate_per_second,
            time_call_recovery_multiplier,
            tempo_drain_scale,
            pressing_drain_scale,
        }
    }

    pub fn base_drain_per_live_second(&self) -> f64 {
        self.base_drain_per_live_second
    }

    pub fn participation_drain_bonus(&self) -> f64 {
        self.participation_drain_bonus
    }

    pub fn dead_ball_recovery_rate_per_second(&self) -> f64 {
        self.dead_ball_recovery_rate_per_second
    }

    pub fn time_call_recovery_multiplier(&self) -> f64 {
        self.time_call_recovery_multiplier
    }

    pub fn tempo_drain_scale(&self) -> f64 {
        self.tempo_drain_scale
    }

    pub fn pressing_drain_scale(&self) -> f64 {
        self.pressing_drain_scale
    }
}

impl Default for EnergyTuningProfile {
    fn default() -> Self {
        Self {
            base_drain_per_live_second: 0.00010,
            participation_drain_bonus: 0.0035,
            dead_ball_recovery_rate_per_second: 0.00012,
            time_call_recovery_multiplier: 1.6,
            tempo_drain_scale: 0.20,
            pressing_drain_scale: 0.20,
        }
    }
}
