use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchSquadSlotDto {
    pub player_id: String,
    pub player_name: String,
    pub squad_number: Option<i32>,
    pub position: String,
    pub slot_role: Option<String>,
    pub was_starter: bool,
    pub was_used: bool,
    pub formation_slot_index: Option<i32>,
    pub pitch_length_ratio: Option<f64>,
    pub pitch_width_ratio: Option<f64>,
    pub minutes_played: u32,
    pub end_energy_level: Option<f64>,
    pub goal_points_scored: u32,
    pub field_goals_scored: u32,
    pub field_points_scored: u32,
    pub goalpoint_assists: u32,
    pub substituted_by_player_id: Option<String>,
    pub substituted_by_player_name: Option<String>,
    pub substituted_in_for_player_id: Option<String>,
    pub substituted_in_for_player_name: Option<String>,
    pub substitution_minute: Option<String>,
    pub substitution_reason: Option<String>,
    pub final_availability_status: String,
}
