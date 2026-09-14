use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FixtureResult {
    home_score: u32,
    away_score: u32,
    winner_team_id: Option<Uuid>,
}

impl FixtureResult {
    pub fn new(home_score: u32, away_score: u32, winner_team_id: Option<Uuid>) -> Self {
        Self {
            home_score,
            away_score,
            winner_team_id,
        }
    }

    pub fn home_score(&self) -> u32 {
        self.home_score
    }

    pub fn away_score(&self) -> u32 {
        self.away_score
    }

    pub fn winner_team_id(&self) -> Option<Uuid> {
        self.winner_team_id
    }
}
