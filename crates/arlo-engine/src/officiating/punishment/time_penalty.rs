use crate::world_state::match_state::availability::AvailabilityState;
use crate::world_state::match_state::state::MatchState;
use arlo_domain::sport_constants::DEFAULT_TIME_PENALTY_MINUTES;
use uuid::Uuid;

pub fn apply_time_penalty(
    state: &mut MatchState,
    offending_player_id: Uuid,
    magnitude: Option<i32>,
) -> AvailabilityState {
    let previous_state = state.availability_for(&offending_player_id);
    let minutes = magnitude.unwrap_or(DEFAULT_TIME_PENALTY_MINUTES).max(1) as f64;
    let seconds = minutes * 60.0;
    state.suspend_player(offending_player_id, seconds);
    previous_state
}