use crate::domain::competition::Competition;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct League {
    competition: Competition,
    division_index: u32,
}

impl League {
    pub fn new(competition: Competition, division_index: u32) -> Self {
        Self {
            competition,
            division_index,
        }
    }

    pub fn id(&self) -> Uuid {
        self.competition.id()
    }

    pub fn competition(&self) -> &Competition {
        &self.competition
    }

    pub fn division_index(&self) -> u32 {
        self.division_index
    }
}
