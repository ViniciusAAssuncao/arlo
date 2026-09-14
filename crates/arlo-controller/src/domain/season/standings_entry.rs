use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StandingsEntry {
    team_id: Uuid,
    played: u32,
    won: u32,
    drawn: u32,
    lost: u32,
    points: i32,
}

impl StandingsEntry {
    pub fn new(
        team_id: Uuid,
        played: u32,
        won: u32,
        drawn: u32,
        lost: u32,
        points: i32,
    ) -> Self {
        Self {
            team_id,
            played,
            won,
            drawn,
            lost,
            points,
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn played(&self) -> u32 {
        self.played
    }

    pub fn won(&self) -> u32 {
        self.won
    }

    pub fn drawn(&self) -> u32 {
        self.drawn
    }

    pub fn lost(&self) -> u32 {
        self.lost
    }

    pub fn points(&self) -> i32 {
        self.points
    }
}
