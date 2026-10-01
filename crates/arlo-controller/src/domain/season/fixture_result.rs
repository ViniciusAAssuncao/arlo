use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FixtureResult {
    home_score: u32,
    away_score: u32,
    home_goal_points: u32,
    away_goal_points: u32,
    home_field_goals: u32,
    away_field_goals: u32,
    home_field_points: u32,
    away_field_points: u32,
    winner_team_id: Option<Uuid>,
}

impl FixtureResult {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        home_score: u32,
        away_score: u32,
        home_goal_points: u32,
        away_goal_points: u32,
        home_field_goals: u32,
        away_field_goals: u32,
        home_field_points: u32,
        away_field_points: u32,
        winner_team_id: Option<Uuid>,
    ) -> Self {
        Self {
            home_score,
            away_score,
            home_goal_points,
            away_goal_points,
            home_field_goals,
            away_field_goals,
            home_field_points,
            away_field_points,
            winner_team_id,
        }
    }

    pub fn home_score(&self) -> u32 {
        self.home_score
    }

    pub fn away_score(&self) -> u32 {
        self.away_score
    }

    pub fn home_goal_points(&self) -> u32 {
        self.home_goal_points
    }

    pub fn away_goal_points(&self) -> u32 {
        self.away_goal_points
    }

    pub fn home_field_goals(&self) -> u32 {
        self.home_field_goals
    }

    pub fn away_field_goals(&self) -> u32 {
        self.away_field_goals
    }

    pub fn home_field_points(&self) -> u32 {
        self.home_field_points
    }

    pub fn away_field_points(&self) -> u32 {
        self.away_field_points
    }

    pub fn winner_team_id(&self) -> Option<Uuid> {
        self.winner_team_id
    }
}
