use crate::services::r#match::QuarterScoreSummary;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuarterScoreDto {
    pub period: u32,
    pub home_points: u32,
    pub away_points: u32,
    pub home_goal_points: u32,
    pub away_goal_points: u32,
    pub home_field_points: u32,
    pub away_field_points: u32,
    pub home_field_goals: u32,
    pub away_field_goals: u32,
    pub added_time_seconds: f64,
    pub has_added_time: bool,
}

impl From<QuarterScoreSummary> for QuarterScoreDto {
    fn from(s: QuarterScoreSummary) -> Self {
        Self {
            period: s.period,
            home_points: s.home_points,
            away_points: s.away_points,
            home_goal_points: s.home_goal_points,
            away_goal_points: s.away_goal_points,
            home_field_points: s.home_field_points,
            away_field_points: s.away_field_points,
            home_field_goals: s.home_field_goals,
            away_field_goals: s.away_field_goals,
            added_time_seconds: s.added_time_seconds,
            has_added_time: s.has_added_time,
        }
    }
}
