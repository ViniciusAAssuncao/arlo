use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct InjuryTuningProfile {
    target_injuries_per_team_per_match: f64,
    contact_share: f64,
    base_contact_hazard_per_collision: f64,
    base_exertion_hazard_per_second: f64,
    fatigue_hazard_multiplier: f64,
    age_hazard_multiplier: f64,
}

impl InjuryTuningProfile {
    pub fn new(
        target_injuries_per_team_per_match: f64,
        contact_share: f64,
        base_contact_hazard_per_collision: f64,
        base_exertion_hazard_per_second: f64,
        fatigue_hazard_multiplier: f64,
        age_hazard_multiplier: f64,
    ) -> Self {
        Self {
            target_injuries_per_team_per_match,
            contact_share,
            base_contact_hazard_per_collision,
            base_exertion_hazard_per_second,
            fatigue_hazard_multiplier,
            age_hazard_multiplier,
        }
    }

    pub fn target_injuries_per_team_per_match(&self) -> f64 {
        self.target_injuries_per_team_per_match
    }

    pub fn contact_share(&self) -> f64 {
        self.contact_share
    }

    pub fn base_contact_hazard_per_collision(&self) -> f64 {
        self.base_contact_hazard_per_collision
    }

    pub fn base_exertion_hazard_per_second(&self) -> f64 {
        self.base_exertion_hazard_per_second
    }

    pub fn fatigue_hazard_multiplier(&self) -> f64 {
        self.fatigue_hazard_multiplier
    }

    pub fn age_hazard_multiplier(&self) -> f64 {
        self.age_hazard_multiplier
    }
}

impl Default for InjuryTuningProfile {
    fn default() -> Self {
        Self {
            target_injuries_per_team_per_match: 1.0,
            contact_share: 0.65,
            base_contact_hazard_per_collision: 0.0025,
            base_exertion_hazard_per_second: 0.0000095,
            fatigue_hazard_multiplier: 1.50,
            age_hazard_multiplier: 1.25,
        }
    }
}
