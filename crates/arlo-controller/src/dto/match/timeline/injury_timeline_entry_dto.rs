use arlo_persistence::models::incidents::MatchInjuryRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InjuryTimelineEntryDto {
    pub sequence_number: u64,
    pub period: u32,
    pub seconds_in_period: f64,
    pub total_elapsed_seconds: f64,
    pub formatted_time: String,
    pub player_id: String,
    pub team_id: String,
    pub mechanism: String,
    pub body_region: String,
    pub severity_grade: String,
    pub injury_definition_id: String,
    pub trigger_probability: f64,
}

impl InjuryTimelineEntryDto {
    pub fn from_row(
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        total_elapsed_seconds: f64,
        formatted_time: String,
        row: &MatchInjuryRow,
    ) -> Self {
        Self {
            sequence_number,
            period,
            seconds_in_period,
            total_elapsed_seconds,
            formatted_time,
            player_id: row.player_id.clone(),
            team_id: row.team_id.clone(),
            mechanism: row.mechanism.clone(),
            body_region: row.body_region.clone(),
            severity_grade: row.severity_grade.clone(),
            injury_definition_id: row.injury_definition_id.clone(),
            trigger_probability: row.trigger_probability,
        }
    }
}