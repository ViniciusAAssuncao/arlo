use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerScoringStatsDto {
    pub attempts: u32,
    pub converted: u32,
    pub missed: u32,
    pub conversion_rate: f64,
    pub goal_points_scored: u32,
    pub field_points_scored: u32,
    pub field_goals_scored: u32,
    pub total_points_scored: u32,
}
