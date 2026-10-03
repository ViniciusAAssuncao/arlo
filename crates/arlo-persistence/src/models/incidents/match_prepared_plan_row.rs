use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct MatchPreparedPlanRow {
    pub match_id: String,
    pub team_id: String,
    pub plan_id: String,
    #[sqlx(json)]
    pub plan: arlo_tactics::PreparedTacticalPlan,
    #[sqlx(json)]
    pub profile: arlo_tactics::TeamTacticalProfile,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct MatchTacticalPlanActivationRow {
    pub match_id: String,
    pub sequence_number: i64,
    pub team_id: String,
    pub plan_id: String,
    pub plan_name: String,
    pub formation_id: String,
    pub profile_id: String,
    pub period: i32,
    pub seconds_in_period: f64,
    pub total_elapsed_seconds: f64,
    #[sqlx(json)]
    pub assignments: Vec<arlo_events::TacticalAssignment>,
}
