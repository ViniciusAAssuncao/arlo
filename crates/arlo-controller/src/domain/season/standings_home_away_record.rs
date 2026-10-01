use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HomeAwayRecord {
    home_won: u32,
    home_drawn: u32,
    home_lost: u32,
    away_won: u32,
    away_drawn: u32,
    away_lost: u32,
}

impl HomeAwayRecord {
    pub fn new(
        home_won: u32,
        home_drawn: u32,
        home_lost: u32,
        away_won: u32,
        away_drawn: u32,
        away_lost: u32,
    ) -> Self {
        Self {
            home_won,
            home_drawn,
            home_lost,
            away_won,
            away_drawn,
            away_lost,
        }
    }

    pub fn home_won(&self) -> u32 {
        self.home_won
    }

    pub fn home_drawn(&self) -> u32 {
        self.home_drawn
    }

    pub fn home_lost(&self) -> u32 {
        self.home_lost
    }

    pub fn away_won(&self) -> u32 {
        self.away_won
    }

    pub fn away_drawn(&self) -> u32 {
        self.away_drawn
    }

    pub fn away_lost(&self) -> u32 {
        self.away_lost
    }
}
