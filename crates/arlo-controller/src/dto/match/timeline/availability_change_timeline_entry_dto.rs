use arlo_persistence::models::incidents::MatchAvailabilityChangeRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailabilityChangeTimelineEntryDto {
    pub sequence_number: u64,
    pub period: u32,
    pub seconds_in_period: f64,
    pub total_elapsed_seconds: f64,
    pub formatted_time: String,
    pub player_id: String,
    pub team_id: String,
    pub previous_status: String,
    pub new_status: String,
    pub remaining_seconds: Option<f64>,
}

impl AvailabilityChangeTimelineEntryDto {
    pub fn from_row(
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        total_elapsed_seconds: f64,
        formatted_time: String,
        row: &MatchAvailabilityChangeRow,
    ) -> Self {
        Self {
            sequence_number,
            period,
            seconds_in_period,
            total_elapsed_seconds,
            formatted_time,
            player_id: row.player_id.clone(),
            team_id: row.team_id.clone(),
            previous_status: row.previous_status.clone(),
            new_status: row.new_status.clone(),
            remaining_seconds: row.remaining_seconds,
        }
    }
}