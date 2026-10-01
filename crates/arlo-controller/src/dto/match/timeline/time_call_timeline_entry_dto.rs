use arlo_persistence::models::incidents::MatchTimeCallRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeCallTimelineEntryDto {
    pub sequence_number: u64,
    pub period: u32,
    pub seconds_in_period: f64,
    pub total_elapsed_seconds: f64,
    pub formatted_time: String,
    pub team_id: String,
    pub remaining_time_calls_after: i32,
    pub reason: String,
}

impl TimeCallTimelineEntryDto {
    pub fn from_row(
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        total_elapsed_seconds: f64,
        formatted_time: String,
        row: &MatchTimeCallRow,
    ) -> Self {
        Self {
            sequence_number,
            period,
            seconds_in_period,
            total_elapsed_seconds,
            formatted_time,
            team_id: row.team_id.clone(),
            remaining_time_calls_after: row.remaining_time_calls_after,
            reason: row.reason.clone(),
        }
    }
}
