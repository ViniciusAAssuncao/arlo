use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LeadPressureProfile {
    weight: f64,
}

impl LeadPressureProfile {
    pub fn new(weight: f64) -> Self {
        Self {
            weight: weight.clamp(0.0, 1.0),
        }
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }
}

impl Default for LeadPressureProfile {
    fn default() -> Self {
        Self { weight: 0.0 }
    }
}

pub fn calculate_lead_pressure(score_deficit: i32, time_urgency: f64, profile: &LeadPressureProfile) -> f64 {
    if score_deficit >= 0 || profile.weight <= 0.0 {
        return 0.0;
    }
    let lead = (-score_deficit) as f64;
    (lead * 0.15 * time_urgency * profile.weight).clamp(0.0, 1.0)
}
