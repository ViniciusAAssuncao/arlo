use arlo_persistence::models::incidents::MatchFoulRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoulTimelineEntryDto {
    pub sequence_number: u64,
    pub period: u32,
    pub seconds_in_period: f64,
    pub total_elapsed_seconds: f64,
    pub formatted_time: String,
    pub offending_player_id: String,
    pub offending_team_id: String,
    pub opposing_player_id: String,
    pub opposing_team_id: String,
    pub origin: String,
    pub original_call_correct: bool,
    pub peace_referee_intervened: bool,
    pub fault_definition_id: Option<String>,
    pub punishment_kind: Option<String>,
    pub punishment_magnitude: Option<i32>,
}

impl FoulTimelineEntryDto {
    pub fn from_row(
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        total_elapsed_seconds: f64,
        formatted_time: String,
        row: &MatchFoulRow,
    ) -> Self {
        Self {
            sequence_number,
            period,
            seconds_in_period,
            total_elapsed_seconds,
            formatted_time,
            offending_player_id: row.offending_player_id.clone(),
            offending_team_id: row.offending_team_id.clone(),
            opposing_player_id: row.opposing_player_id.clone(),
            opposing_team_id: row.opposing_team_id.clone(),
            origin: row.origin.clone(),
            original_call_correct: row.original_call_correct,
            peace_referee_intervened: row.peace_referee_intervened,
            fault_definition_id: row.fault_definition_id.clone(),
            punishment_kind: row.punishment_kind.clone(),
            punishment_magnitude: row.punishment_magnitude,
        }
    }
}
