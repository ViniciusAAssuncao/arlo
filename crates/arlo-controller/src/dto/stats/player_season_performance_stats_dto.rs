use crate::dto::r#match::PlayerPerformanceBreakdownDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSeasonPerformanceStatsDto {
    pub matches_rated: u32,
    pub average_rating: f64,
    pub average_performance_rating: f64,
    pub average_confidence: f64,
    pub highest_rating: f64,
    pub lowest_rating: f64,
    pub average_breakdown: PlayerPerformanceBreakdownDto,
}