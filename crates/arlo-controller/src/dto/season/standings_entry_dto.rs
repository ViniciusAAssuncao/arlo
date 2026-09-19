use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandingsEntryDto {
    pub team_id: String,
    pub team_name: String,
    pub played: u32,
    pub won: u32,
    pub drawn: u32,
    pub lost: u32,
    pub goal_points_for: u32,
    pub goal_points_against: u32,
    pub field_goals_for: u32,
    pub field_goals_against: u32,
    pub field_points_for: u32,
    pub field_points_against: u32,
    pub total_points_for: u32,
    pub total_points_against: u32,
    pub ispa: f64,
    pub qta: f64,
}