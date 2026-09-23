use arlo_persistence::models::incidents::{MatchKickFoulAwardRow, MatchKickFoulDecisionRow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KickFoulTimelineEntryDto {
    pub sequence_number: u64,
    pub period: u32,
    pub seconds_in_period: f64,
    pub total_elapsed_seconds: f64,
    pub formatted_time: String,
    pub kind: String,
    pub awarded_team_id: Option<String>,
    pub offending_team_id: Option<String>,
    pub scoring_tier: Option<String>,
    pub taker_id: Option<String>,
    pub decision: Option<String>,
}

impl KickFoulTimelineEntryDto {
    pub fn from_award(
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        total_elapsed_seconds: f64,
        formatted_time: String,
        row: &MatchKickFoulAwardRow,
    ) -> Self {
        Self {
            sequence_number,
            period,
            seconds_in_period,
            total_elapsed_seconds,
            formatted_time,
            kind: "Award".to_string(),
            awarded_team_id: Some(row.awarded_team_id.clone()),
            offending_team_id: Some(row.offending_team_id.clone()),
            scoring_tier: Some(row.scoring_tier.clone()),
            taker_id: None,
            decision: None,
        }
    }

    pub fn from_decision(
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        total_elapsed_seconds: f64,
        formatted_time: String,
        row: &MatchKickFoulDecisionRow,
    ) -> Self {
        Self {
            sequence_number,
            period,
            seconds_in_period,
            total_elapsed_seconds,
            formatted_time,
            kind: "Decision".to_string(),
            awarded_team_id: None,
            offending_team_id: None,
            scoring_tier: None,
            taker_id: Some(row.taker_id.clone()),
            decision: Some(row.decision.clone()),
        }
    }
}
