use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerClubHistoryDto {
    pub id: String,
    pub player_id: String,
    pub team_id: String,
    pub team_name: String,
    pub joined_year: i64,
    pub left_year: Option<i64>,
    pub period_display: String,
}
