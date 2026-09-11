use crate::officiating::punishment::reversal::{apply_play_reversal, PlayReversalSnapshot};
use crate::world_state::match_state::state::MatchState;

pub fn apply_invalidate_previous_play(
    state: &mut MatchState,
    pre_play_snapshot: &PlayReversalSnapshot,
) {
    apply_play_reversal(state, pre_play_snapshot);
}