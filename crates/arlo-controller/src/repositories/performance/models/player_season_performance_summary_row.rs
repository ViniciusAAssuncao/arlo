use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct PlayerSeasonPerformanceSummaryRow {
    pub matches_rated: i64,
    pub average_rating: f64,
    pub average_performance_rating: f64,
    pub average_confidence: f64,
    pub highest_rating: f64,
    pub lowest_rating: f64,
    pub avg_execution: f64,
    pub avg_production: f64,
    pub avg_defense: f64,
    pub avg_ball_security: f64,
    pub avg_discipline: f64,
    pub avg_high_impact: f64,
}