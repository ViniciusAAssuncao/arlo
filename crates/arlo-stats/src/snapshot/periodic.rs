use crate::snapshot::player_match_snapshot::PlayerMatchSnapshot;
use crate::snapshot::team_match_snapshot::TeamMatchSnapshot;
use arlo_events::MatchClockInstant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PeriodicMatchSnapshot {
    pub sequence_number: u64,
    pub clock: MatchClockInstant,
    pub player_snapshots: Vec<PlayerMatchSnapshot>,
    pub team_snapshots: Vec<TeamMatchSnapshot>,
}

impl PeriodicMatchSnapshot {
    pub fn new(
        sequence_number: u64,
        clock: MatchClockInstant,
        player_snapshots: Vec<PlayerMatchSnapshot>,
        team_snapshots: Vec<TeamMatchSnapshot>,
    ) -> Self {
        Self {
            sequence_number,
            clock,
            player_snapshots,
            team_snapshots,
        }
    }

    pub fn sequence_number(&self) -> u64 {
        self.sequence_number
    }

    pub fn clock(&self) -> MatchClockInstant {
        self.clock
    }

    pub fn player_snapshots(&self) -> &[PlayerMatchSnapshot] {
        &self.player_snapshots
    }

    pub fn team_snapshots(&self) -> &[TeamMatchSnapshot] {
        &self.team_snapshots
    }
}
