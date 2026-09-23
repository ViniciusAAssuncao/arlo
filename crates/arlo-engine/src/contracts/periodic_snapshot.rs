use crate::contracts::player_snapshot::SimulationPlayerSnapshot;
use crate::contracts::team_snapshot::SimulationTeamSnapshot;
use arlo_events::MatchClockInstant;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationPeriodicSnapshot {
    pub sequence_number: u64,
    pub clock: MatchClockInstant,
    pub player_snapshots: Vec<SimulationPlayerSnapshot>,
    pub team_snapshots: Vec<SimulationTeamSnapshot>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RefereeSnapshotData {
    pub calls_made: u32,
    pub calls_correct: u32,
    pub calls_incorrect: u32,
    pub peace_referee_interventions: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ManagerSnapshotData {
    pub team_id: Uuid,
    pub substitutions_made: u32,
    pub time_calls_used: u32,
    pub challenges_won: u32,
    pub challenges_lost: u32,
    pub tactical_profile_switches: u32,
}