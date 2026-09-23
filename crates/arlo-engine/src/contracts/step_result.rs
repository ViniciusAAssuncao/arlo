use crate::contracts::config::MatchScore;
use crate::contracts::status::SimulationStatus;
use arlo_events::{MatchClockInstant, MatchEventEnvelope};
use arlo_manager_control::RequiredManagerDecision;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationStepResult {
    pub status: SimulationStatus,
    pub events: Vec<MatchEventEnvelope>,
    pub clock: MatchClockInstant,
    pub home_score: MatchScore,
    pub away_score: MatchScore,
    pub pending_decision: Option<RequiredManagerDecision>,
}