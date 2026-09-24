use arlo_persistence::models::incidents::MatchPlayCallSelectionRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayCallTimelineEntryDto {
    pub sequence_number: u64,
    pub period: u32,
    pub seconds_in_period: f64,
    pub total_elapsed_seconds: f64,
    pub formatted_time: String,
    pub team_id: String,
    pub play_call_id: String,
    pub play_call_name: String,
    pub category: String,
}

impl PlayCallTimelineEntryDto {
    pub fn from_row(
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        total_elapsed_seconds: f64,
        formatted_time: String,
        row: &MatchPlayCallSelectionRow,
    ) -> Self {
        Self {
            sequence_number,
            period,
            seconds_in_period,
            total_elapsed_seconds,
            formatted_time,
            team_id: row.team_id.clone(),
            play_call_id: row.play_call_id.clone(),
            play_call_name: row.play_call_name.clone(),
            category: row.category.clone(),
        }
    }
}
