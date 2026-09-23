use crate::contracts::config::MatchScore;
use crate::contracts::periodic_snapshot::SimulationPeriodicSnapshot;
use crate::contracts::player_snapshot::SimulationPlayerSnapshot;
use crate::contracts::status::SimulationStatus;
use crate::contracts::team_snapshot::SimulationTeamSnapshot;
use arlo_events::{MatchClockInstant, MatchEventEnvelope};
use arlo_manager_control::RequiredManagerDecision;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationOutput {
    pub match_id: Uuid,
    pub status: SimulationStatus,
    pub final_clock: MatchClockInstant,
    pub home_score: MatchScore,
    pub away_score: MatchScore,
    pub winner_team_id: Option<Uuid>,
    pub total_events: usize,
    pub total_periods_played: u32,
    pub events: Vec<MatchEventEnvelope>,
    pub player_snapshots: Vec<SimulationPlayerSnapshot>,
    pub team_snapshots: Vec<SimulationTeamSnapshot>,
    pub periodic_snapshots: Vec<SimulationPeriodicSnapshot>,
    pub pending_decision: Option<RequiredManagerDecision>,
}

impl SimulationOutput {
    pub fn is_completed(&self) -> bool {
        self.status.is_completed()
    }

    pub fn is_draw(&self) -> bool {
        self.winner_team_id.is_none() && self.is_completed()
    }
}