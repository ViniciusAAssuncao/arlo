use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPerformanceBreakdownDto {
    pub execution: f64,
    pub production: f64,
    pub defense: f64,
    pub ball_security: f64,
    pub discipline: f64,
    pub high_impact: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMatchPerformanceDto {
    pub player_id: String,
    pub player_name: Option<String>,
    pub team_id: String,
    pub offensive_position: String,
    pub defensive_position: String,
    pub slot_role: String,
    pub rating: f64,
    pub performance_rating: f64,
    pub outcome_adjustment: f64,
    pub confidence: f64,
    pub seconds_played: f64,
    pub effective_opportunities: u32,
    pub breakdown: PlayerPerformanceBreakdownDto,
    pub model_version: u32,
}
