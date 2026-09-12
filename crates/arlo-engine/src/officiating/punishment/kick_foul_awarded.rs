use crate::kick_foul::{determine_kick_foul_scoring_tier, KickFoulPending};
use crate::world_state::match_state::state::MatchState;
use uuid::Uuid;

pub fn apply_kick_foul_awarded(state: &mut MatchState, offending_team_id: Uuid) {
    let awarded_team_id = if offending_team_id == state.home_team_id() {
        state.away_team_id()
    } else {
        state.home_team_id()
    };
    let spot = state.possession().scrimmage_point();
    let scoring_tier = determine_kick_foul_scoring_tier(state.pitch(), spot);
    let pending = KickFoulPending::new(awarded_team_id, spot, scoring_tier);
    state.set_kick_foul_pending(pending);
}
