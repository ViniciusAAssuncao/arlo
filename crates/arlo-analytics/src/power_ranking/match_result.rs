use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PowerMatchResult {
    pub fixture_id: Uuid,
    pub played_year: i64,
    pub played_day_of_year: u32,
    pub home_team_id: Uuid,
    pub away_team_id: Uuid,
    pub home_score: u32,
    pub away_score: u32,
    pub neutral_venue: bool,
}

impl PowerMatchResult {
    pub fn ordering_key(&self) -> (i64, u32, Uuid) {
        (self.played_year, self.played_day_of_year, self.fixture_id)
    }

    pub fn home_result(&self) -> f64 {
        match self.home_score.cmp(&self.away_score) {
            std::cmp::Ordering::Greater => 1.0,
            std::cmp::Ordering::Equal => 0.5,
            std::cmp::Ordering::Less => 0.0,
        }
    }

    pub fn score_margin(&self) -> u32 {
        self.home_score.abs_diff(self.away_score)
    }
}
