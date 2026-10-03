use super::player_performance_dto::PlayerMatchPerformanceDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchPerformanceSummaryDto {
    pub match_id: String,
    pub home_average_rating: f64,
    pub away_average_rating: f64,
    pub home_ratings: Vec<PlayerMatchPerformanceDto>,
    pub away_ratings: Vec<PlayerMatchPerformanceDto>,
    pub match_mvp: Option<PlayerMatchPerformanceDto>,
}