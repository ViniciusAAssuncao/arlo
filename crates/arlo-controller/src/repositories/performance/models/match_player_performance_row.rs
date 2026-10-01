use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct MatchPlayerPerformanceRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub team_id: String,
    pub offensive_position: String,
    pub defensive_position: String,
    pub slot_role: String,
    pub performance_rating: f64,
    pub outcome_adjustment: f64,
    pub final_rating: f64,
    pub confidence: f64,
    pub seconds_played: f64,
    pub effective_opportunities: i32,
    pub execution: f64,
    pub production: f64,
    pub defense: f64,
    pub ball_security: f64,
    pub discipline: f64,
    pub high_impact: f64,
    pub model_version: String,
}