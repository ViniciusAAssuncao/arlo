use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamPossessionSummaryDto {
    pub team_id: String,
    pub possession_seconds: f64,
    pub possession_percentage: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamImpulseSummaryDto {
    pub team_id: String,
    pub average_baseline: f64,
    pub current_average_value: f64,
    pub min_average_value: f64,
    pub max_average_value: f64,
    pub average_value: f64,
    pub time_below_baseline_seconds: f64,
    pub runs_count: i32,
    pub longest_run_duration_seconds: f64,
    pub peak_run_average_value: f64,
    pub total_integrated_run_intensity: f64,
    pub average_run_duration_seconds: f64,
    pub average_run_intensity: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchTeamStatsDto {
    pub home_possession: TeamPossessionSummaryDto,
    pub away_possession: TeamPossessionSummaryDto,
    pub home_impulse: Option<TeamImpulseSummaryDto>,
    pub away_impulse: Option<TeamImpulseSummaryDto>,
}
