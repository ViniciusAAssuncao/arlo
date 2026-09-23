use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationTeamSnapshot {
    pub team_id: Uuid,
    pub total_possession_seconds: f64,
}

impl SimulationTeamSnapshot {
    pub fn new(team_id: Uuid, total_possession_seconds: f64) -> Self {
        Self {
            team_id,
            total_possession_seconds,
        }
    }
}