use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct MatchTacticalRealignmentRow {
    pub match_id: String,
    pub sequence_number: i64,
    pub team_id: String,
    pub period: i32,
    pub seconds_in_period: f64,
    pub total_elapsed_seconds: f64,
    #[sqlx(json)]
    pub assignments: [arlo_events::TacticalAssignment; 2],
}
