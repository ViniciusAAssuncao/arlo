use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PossessionRole {
    offense: Uuid,
    defense: Uuid,
}

impl PossessionRole {
    pub fn new(offense: Uuid, defense: Uuid) -> Self {
        Self { offense, defense }
    }

    pub fn offense(&self) -> Uuid {
        self.offense
    }

    pub fn defense(&self) -> Uuid {
        self.defense
    }

    pub fn is_offense(&self, team_id: Uuid) -> bool {
        self.offense == team_id
    }

    pub fn is_defense(&self, team_id: Uuid) -> bool {
        self.defense == team_id
    }

    pub fn swap(&self) -> Self {
        Self {
            offense: self.defense,
            defense: self.offense,
        }
    }
}

pub fn opening_possession(home_team: Uuid, away_team: Uuid) -> PossessionRole {
    PossessionRole::new(home_team, away_team)
}
