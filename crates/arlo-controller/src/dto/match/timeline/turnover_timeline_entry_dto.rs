use arlo_persistence::models::incidents::MatchTurnoverRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnoverTimelineEntryDto {
    pub sequence_number: u64,
    pub period: u32,
    pub seconds_in_period: f64,
    pub total_elapsed_seconds: f64,
    pub formatted_time: String,
    pub previous_offense_team_id: String,
    pub new_offense_team_id: String,
    pub recovering_player_id: Option<String>,
    pub lost_by_player_id: Option<String>,
    pub in_live_play: bool,
}

impl TurnoverTimelineEntryDto {
    pub fn from_row(
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        total_elapsed_seconds: f64,
        formatted_time: String,
        row: &MatchTurnoverRow,
    ) -> Self {
        Self {
            sequence_number,
            period,
            seconds_in_period,
            total_elapsed_seconds,
            formatted_time,
            previous_offense_team_id: row.previous_offense_team_id.clone(),
            new_offense_team_id: row.new_offense_team_id.clone(),
            recovering_player_id: row.recovering_player_id.clone(),
            lost_by_player_id: row.lost_by_player_id.clone(),
            in_live_play: row.in_live_play,
        }
    }
}
