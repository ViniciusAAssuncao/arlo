use arlo_events::TacticalAssignment;
use arlo_persistence::models::MatchTacticalPlanActivationRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TacticalPlanTimelineEntryDto {
    pub sequence_number: u64,
    pub period: u32,
    pub seconds_in_period: f64,
    pub total_elapsed_seconds: f64,
    pub formatted_time: String,
    pub team_id: String,
    pub plan_id: String,
    pub plan_name: String,
    pub formation_id: String,
    pub profile_id: String,
    pub assignments: Vec<TacticalAssignment>,
}

impl TacticalPlanTimelineEntryDto {
    pub fn from_row(row: &MatchTacticalPlanActivationRow, formatted_time: String) -> Self {
        Self {
            sequence_number: row.sequence_number.max(0) as u64,
            period: row.period.max(0) as u32,
            seconds_in_period: row.seconds_in_period,
            total_elapsed_seconds: row.total_elapsed_seconds,
            formatted_time,
            team_id: row.team_id.clone(),
            plan_id: row.plan_id.clone(),
            plan_name: row.plan_name.clone(),
            formation_id: row.formation_id.clone(),
            profile_id: row.profile_id.clone(),
            assignments: row.assignments.clone(),
        }
    }
}
