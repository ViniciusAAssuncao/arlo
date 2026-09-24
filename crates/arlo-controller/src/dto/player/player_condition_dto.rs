use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMedicalConditionDto {
    pub player_id: String,
    pub energy_level: f64,
    pub anaerobic_reserve: f64,
    pub impulse_value: u8,
    pub impulse_baseline: f64,
    pub conditioning_score: f64,
    pub status: String,
    pub is_injured: bool,
    pub injury_name: Option<String>,
    pub body_region: Option<String>,
    pub severity_grade: Option<String>,
    pub days_remaining: Option<u32>,
    pub observation_days_remaining: Option<u32>,
    pub expected_recovery_days: Option<u32>,
    pub is_relapse: bool,
    pub readiness_score: f64,
    pub readiness_level: String,
    pub caution_recommended: bool,
}
