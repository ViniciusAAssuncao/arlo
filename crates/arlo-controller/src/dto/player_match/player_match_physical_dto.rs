use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMatchPhysicalDto {
    pub end_energy_level: f64,
    pub peak_anaerobic_depletion: f64,
    pub total_distance_covered: f64,
    pub intra_match_recovery_amount: f64,
}