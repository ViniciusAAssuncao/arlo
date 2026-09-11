use crate::world_state::match_state::availability::AvailabilityState;
use crate::world_state::match_state::state::MatchState;
use uuid::Uuid;

pub fn apply_expulsion(state: &mut MatchState, offending_player_id: Uuid) -> AvailabilityState {
    let previous_state = state.availability_for(&offending_player_id);
    state.expel_player(offending_player_id);
    previous_state
}