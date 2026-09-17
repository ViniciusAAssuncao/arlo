use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixtureSummaryDto {
    pub id: String,
    pub round_index: u32,
    pub home_team_name: String,
    pub away_team_name: String,
    pub status: String,
    pub home_score: Option<i32>,
    pub away_score: Option<i32>,
    pub scheduled_year: i64,
    pub scheduled_day_of_year: u32,
    pub scheduled_month_name: String,
    pub scheduled_day_of_month: u32,
    pub scheduled_week_day_name: String,
    pub venue_name: Option<String>,
}