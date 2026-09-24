use super::quarter_score_dto::QuarterScoreDto;
use crate::dto::team::TeamVenueDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchTeamScoreDto {
    pub team_id: String,
    pub team_name: String,
    pub is_home: bool,
    pub primary_color_hex: Option<String>,
    pub secondary_color_hex: Option<String>,
    pub total_points: i32,
    pub goal_points: i32,
    pub field_goals: i32,
    pub field_points: i32,
    pub formatted_score: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchSummaryDto {
    pub match_id: String,
    pub fixture_id: Option<String>,
    pub home_team: MatchTeamScoreDto,
    pub away_team: MatchTeamScoreDto,
    pub venue: Option<TeamVenueDto>,
    pub head_referee_id: String,
    pub head_referee_name: String,
    pub peace_referee_id: String,
    pub peace_referee_name: String,
    pub quarter_scores: Vec<QuarterScoreDto>,
    pub final_period: u32,
    pub went_to_overtime: bool,
    pub attendance: Option<u32>,
    pub completed_at_unix_seconds: i64,
}
