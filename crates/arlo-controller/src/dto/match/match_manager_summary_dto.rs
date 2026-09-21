use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagerSubstitutionReasonDto {
    pub reason: String,
    pub count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagerPlayCallCategoryDto {
    pub category: String,
    pub count: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchPlayCallOutcomeSummaryDto {
    pub play_call_id: String,
    pub play_call_name: String,
    pub attempts: u32,
    pub successes: u32,
    pub success_rate: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamManagerSummaryDto {
    pub team_id: String,
    pub manager_id: Option<String>,
    pub manager_name: Option<String>,
    pub substitutions_made: u32,
    pub time_calls_used: u32,
    pub challenges_won: u32,
    pub challenges_lost: u32,
    pub tactical_profile_switches: u32,
    pub substitutions_by_reason: Vec<ManagerSubstitutionReasonDto>,
    pub play_calls_by_category: Vec<ManagerPlayCallCategoryDto>,
    pub play_call_outcomes: Vec<MatchPlayCallOutcomeSummaryDto>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchManagerSummaryDto {
    pub home_manager: Option<TeamManagerSummaryDto>,
    pub away_manager: Option<TeamManagerSummaryDto>,
}