use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MatchOutcome {
    Win,
    Draw,
    Loss,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamFormEntryDto {
    pub fixture_id: String,
    pub match_id: Option<String>,
    pub opponent_id: String,
    pub opponent_name: String,
    pub is_home: bool,
    pub outcome: MatchOutcome,
    pub outcome_code: String,
    pub score_display: String,
    pub team_score: i32,
    pub opponent_score: i32,
    pub scheduled_year: i64,
    pub scheduled_day_of_year: u32,
    pub scheduled_month_name: String,
    pub scheduled_day_of_month: u32,
    pub scheduled_week_day_name: String,
    pub venue_name: Option<String>,
}
