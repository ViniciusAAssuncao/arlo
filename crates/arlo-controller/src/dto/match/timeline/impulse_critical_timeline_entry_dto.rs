use arlo_persistence::models::incidents::MatchImpulseCriticalEventRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImpulseCriticalTimelineEntryDto {
    pub sequence_number: u64,
    pub period: u32,
    pub seconds_in_period: f64,
    pub total_elapsed_seconds: f64,
    pub formatted_time: String,
    pub player_id: String,
    pub value: i32,
    pub duration_seconds: f64,
}

impl ImpulseCriticalTimelineEntryDto {
    pub fn from_row(
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        total_elapsed_seconds: f64,
        formatted_time: String,
        row: &MatchImpulseCriticalEventRow,
    ) -> Self {
        Self {
            sequence_number,
            period,
            seconds_in_period,
            total_elapsed_seconds,
            formatted_time,
            player_id: row.player_id.clone(),
            value: row.value,
            duration_seconds: row.duration_seconds,
        }
    }
}
