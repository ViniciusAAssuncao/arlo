use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ReadinessTuningProfile {
    pub recent_return_window_days: u32,
    pub energy_weight: f64,
    pub anaerobic_weight: f64,
    pub conditioning_weight: f64,
    pub active_observation_penalty: f64,
    pub active_injury_penalty: f64,
    pub recent_return_max_penalty: f64,
    pub fully_fit_threshold: f64,
    pub caution_threshold: f64,
    pub high_risk_threshold: f64,
    pub caution_ability_tolerance: i32,
    pub caution_min_position_proficiency: i32,
    pub max_anaerobic_caution_reduction: f64,
}

impl Default for ReadinessTuningProfile {
    fn default() -> Self {
        Self {
            recent_return_window_days: 14,
            energy_weight: 0.35,
            anaerobic_weight: 0.15,
            conditioning_weight: 0.50,
            active_observation_penalty: 0.40,
            active_injury_penalty: 0.90,
            recent_return_max_penalty: 0.45,
            fully_fit_threshold: 0.80,
            caution_threshold: 0.55,
            high_risk_threshold: 0.35,
            caution_ability_tolerance: 8,
            caution_min_position_proficiency: 12,
            max_anaerobic_caution_reduction: 0.35,
        }
    }
}
