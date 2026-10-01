use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BracketSeed {
    seed_number: u32,
    team_id: Uuid,
}

impl BracketSeed {
    pub fn new(seed_number: u32, team_id: Uuid) -> Self {
        Self {
            seed_number,
            team_id,
        }
    }

    pub fn seed_number(&self) -> u32 {
        self.seed_number
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }
}
