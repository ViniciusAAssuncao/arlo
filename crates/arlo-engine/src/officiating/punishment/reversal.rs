use crate::possession::PossessionSnapshot;
use crate::world_state::match_state::score::TeamScore;
use crate::world_state::match_state::state::MatchState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayReversalSnapshot {
    pub possession: PossessionSnapshot,
    pub home_score: TeamScore,
    pub away_score: TeamScore,
    pub drives_in_current_series: u32,
}

impl PlayReversalSnapshot {
    pub fn new(
        possession: PossessionSnapshot,
        home_score: TeamScore,
        away_score: TeamScore,
        drives_in_current_series: u32,
    ) -> Self {
        Self {
            possession,
            home_score,
            away_score,
            drives_in_current_series,
        }
    }
}

pub fn capture_play_reversal_snapshot(state: &MatchState) -> PlayReversalSnapshot {
    PlayReversalSnapshot {
        possession: state.possession().clone(),
        home_score: state.home_score(),
        away_score: state.away_score(),
        drives_in_current_series: state.drives_in_current_series(),
    }
}

pub fn apply_play_reversal(state: &mut MatchState, snapshot: &PlayReversalSnapshot) {
    *state.possession_mut() = snapshot.possession.clone();
    state.restore_scoreboard(
        snapshot.home_score,
        snapshot.away_score,
        snapshot.drives_in_current_series,
    );
}