use arlo_persistence::models::incidents::MatchChallengeRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeTimelineEntryDto {
    pub sequence_number: u64,
    pub period: u32,
    pub seconds_in_period: f64,
    pub total_elapsed_seconds: f64,
    pub formatted_time: String,
    pub team_id: String,
    pub call_kind: String,
    pub success: bool,
    pub remaining_challenges_after: i32,
}

impl ChallengeTimelineEntryDto {
    pub fn from_row(
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        total_elapsed_seconds: f64,
        formatted_time: String,
        row: &MatchChallengeRow,
    ) -> Self {
        Self {
            sequence_number,
            period,
            seconds_in_period,
            total_elapsed_seconds,
            formatted_time,
            team_id: row.team_id.clone(),
            call_kind: row.call_kind.clone(),
            success: row.success,
            remaining_challenges_after: row.remaining_challenges_after,
        }
    }
}