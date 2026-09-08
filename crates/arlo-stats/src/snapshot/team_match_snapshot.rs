use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamMatchSnapshot {
    pub team_id: Uuid,
    pub total_possession_seconds: f64,
}

impl TeamMatchSnapshot {
    pub fn new(team_id: Uuid, total_possession_seconds: f64) -> Self {
        Self {
            team_id,
            total_possession_seconds,
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn total_possession_seconds(&self) -> f64 {
        self.total_possession_seconds
    }
}

impl Default for TeamMatchSnapshot {
    fn default() -> Self {
        Self {
            team_id: Uuid::nil(),
            total_possession_seconds: 0.0,
        }
    }
}