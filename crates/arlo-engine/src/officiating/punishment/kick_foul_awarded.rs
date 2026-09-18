use crate::kick_foul::{determine_kick_foul_scoring_tier, KickFoulPending};
use crate::possession::PitchState;
use crate::world_state::match_state::state::MatchState;
use uuid::Uuid;

pub fn apply_kick_foul_awarded(state: &mut MatchState, offending_team_id: Uuid) {
    let awarded_team_id = if offending_team_id == state.home_team_id() {
        state.away_team_id()
    } else {
        state.home_team_id()
    };
    let spot_x = state.possession().scrimmage_x_mirim();
    let spot_y = state.pitch().width_mirim() * 0.5;
    let pitch_len = state.pitch().length_mirim();
    let is_home = awarded_team_id == state.home_team_id();
    let norm_prox = if is_home {
        (spot_x / pitch_len.max(1.0)).clamp(0.0, 1.0)
    } else {
        ((pitch_len - spot_x) / pitch_len.max(1.0)).clamp(0.0, 1.0)
    };
    let zone = PitchState::determine_zone_from_proximity(norm_prox);
    let scoring_tier = determine_kick_foul_scoring_tier(zone);
    let pending = KickFoulPending::new(awarded_team_id, spot_x, spot_y, scoring_tier);
    state.set_kick_foul_pending(pending);
}
