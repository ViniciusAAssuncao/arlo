use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefereePerformanceSummaryDto {
    pub referee_id: String,
    pub referee_name: String,
    pub role: String,
    pub calls_made: i32,
    pub calls_correct: i32,
    pub calls_incorrect: i32,
    pub accuracy_rate: f64,
    pub peace_referee_interventions: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoulOriginSummaryDto {
    pub origin: String,
    pub count: u32,
    pub correct_count: u32,
    pub incorrect_count: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchOfficiatingDto {
    pub head_referee: Option<RefereePerformanceSummaryDto>,
    pub peace_referee: Option<RefereePerformanceSummaryDto>,
    pub fouls_by_origin: Vec<FoulOriginSummaryDto>,
}