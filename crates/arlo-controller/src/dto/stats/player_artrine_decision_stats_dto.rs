use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerArtrineDecisionStatsDto {
    pub total_decisions: u32,
    pub total_successful_decisions: u32,
    pub total_failed_decisions: u32,
    pub success_rate: f64,
    pub total_mirins_advanced: f64,
    pub average_mirins_per_decision: f64,
    pub total_points_generated: u32,
    pub goal_points_generated: u32,
    pub field_points_generated: u32,
    pub field_goals_generated: u32,
}
