use crate::world_state::match_state::state::MatchState;
use arlo_domain::sport_constants::DEFAULT_LOSS_OF_DRIVE_COUNT;

pub fn apply_loss_of_drive(state: &mut MatchState, magnitude: Option<i32>) {
    let count = magnitude.unwrap_or(DEFAULT_LOSS_OF_DRIVE_COUNT).max(0) as u32;
    for _ in 0..count {
        state.reverse_drive();
    }
}