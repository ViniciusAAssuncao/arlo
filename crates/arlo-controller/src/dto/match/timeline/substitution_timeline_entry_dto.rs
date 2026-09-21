use arlo_persistence::models::incidents::MatchSubstitutionRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstitutionTimelineEntryDto {
    pub sequence_number: u64,
    pub period: u32,
    pub seconds_in_period: f64,
    pub total_elapsed_seconds: f64,
    pub formatted_time: String,
    pub team_id: String,
    pub player_out_id: String,
    pub player_in_id: String,
    pub reason: String,
}

impl SubstitutionTimelineEntryDto {
    pub fn from_row(
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        total_elapsed_seconds: f64,
        formatted_time: String,
        row: &MatchSubstitutionRow,
    ) -> Self {
        Self {
            sequence_number,
            period,
            seconds_in_period,
            total_elapsed_seconds,
            formatted_time,
            team_id: row.team_id.clone(),
            player_out_id: row.player_out_id.clone(),
            player_in_id: row.player_in_id.clone(),
            reason: row.reason.clone(),
        }
    }
}