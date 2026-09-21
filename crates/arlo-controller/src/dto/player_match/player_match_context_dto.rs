use crate::dto::team::MatchOutcome;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMatchContextDto {
    pub was_starter: bool,
    pub was_used: bool,
    pub formation_slot_index: Option<i32>,
    pub slot_role: Option<String>,
    pub minutes_played: u32,
    pub seconds_played: f64,
    pub entry_instant_seconds: Option<f64>,
    pub exit_instant_seconds: Option<f64>,
    pub entry_time_formatted: Option<String>,
    pub exit_time_formatted: Option<String>,
    pub left_due_to_incident: bool,
    pub team_id: String,
    pub team_name: String,
    pub opponent_team_id: String,
    pub opponent_team_name: String,
    pub is_home: bool,
    pub outcome: MatchOutcome,
    pub team_score: i32,
    pub opponent_score: i32,
}