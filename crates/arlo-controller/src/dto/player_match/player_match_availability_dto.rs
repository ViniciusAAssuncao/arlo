use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMatchAvailabilityDto {
    pub final_availability_status: String,
    pub total_suspended_seconds: f64,
    pub final_suspended_remaining_seconds: Option<f64>,
    pub expulsion_count: u32,
    pub is_currently_expelled: bool,
}