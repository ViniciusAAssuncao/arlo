use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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
pub struct PlayerPerformanceCategoryContributionDto {
    pub category: String,
    pub latent_contribution: f64,
    pub rating_latent_contribution: f64,
    pub observations: u32,
    pub opportunity_weight: f64,
    pub rating_opportunity_weight: f64,
    pub positional_relevance: f64,
    pub rating_high_impact: f64,
    pub breakdown: PlayerPerformanceBreakdownDto,
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
    pub is_rated: bool,
    pub performance_rating: f64,
    pub outcome_adjustment: f64,
    pub confidence: f64,
    pub seconds_played: f64,
    pub effective_opportunities: u32,
    pub effective_opportunity_weight: f64,
    pub offensive_latent: f64,
    pub defensive_latent: f64,
    pub raw_latent: f64,
    pub quality_signal: f64,
    pub confidence_evidence: f64,
    pub rating_latent: f64,
    pub impact_signal: f64,
    pub impact_adjustment: f64,
    pub breakdown: PlayerPerformanceBreakdownDto,
    pub category_contributions: Vec<PlayerPerformanceCategoryContributionDto>,
    pub model_version: u32,
}
