use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct FixtureRow {
    pub id: String,
    pub season_stage_id: String,
    pub round_index: i32,
    pub home_team_id: String,
    pub away_team_id: String,
    pub is_neutral_venue: bool,
    pub venue_id: Option<String>,
    pub scheduled_year: i64,
    pub scheduled_day_of_year: i32,
    pub status: String,
    pub home_score: Option<i32>,
    pub away_score: Option<i32>,
    pub home_goal_points: Option<i32>,
    pub away_goal_points: Option<i32>,
    pub home_field_goals: Option<i32>,
    pub away_field_goals: Option<i32>,
    pub home_field_points: Option<i32>,
    pub away_field_points: Option<i32>,
}

impl FixtureRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        season_stage_id: Uuid,
        round_index: u32,
        home_team_id: Uuid,
        away_team_id: Uuid,
        is_neutral_venue: bool,
        venue_id: Option<Uuid>,
        scheduled_year: i64,
        scheduled_day_of_year: u32,
        status: impl Into<String>,
        home_score: Option<u32>,
        away_score: Option<u32>,
        home_goal_points: Option<u32>,
        away_goal_points: Option<u32>,
        home_field_goals: Option<u32>,
        away_field_goals: Option<u32>,
        home_field_points: Option<u32>,
        away_field_points: Option<u32>,
    ) -> Self {
        Self {
            id: id.to_string(),
            season_stage_id: season_stage_id.to_string(),
            round_index: round_index as i32,
            home_team_id: home_team_id.to_string(),
            away_team_id: away_team_id.to_string(),
            is_neutral_venue,
            venue_id: venue_id.map(|v| v.to_string()),
            scheduled_year,
            scheduled_day_of_year: scheduled_day_of_year as i32,
            status: status.into(),
            home_score: home_score.map(|s| s as i32),
            away_score: away_score.map(|s| s as i32),
            home_goal_points: home_goal_points.map(|s| s as i32),
            away_goal_points: away_goal_points.map(|s| s as i32),
            home_field_goals: home_field_goals.map(|s| s as i32),
            away_field_goals: away_field_goals.map(|s| s as i32),
            home_field_points: home_field_points.map(|s| s as i32),
            away_field_points: away_field_points.map(|s| s as i32),
        }
    }
}
