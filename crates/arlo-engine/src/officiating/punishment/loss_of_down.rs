use crate::possession::PossessionSnapshot;
use crate::world_state::match_state::state::MatchState;
use arlo_domain::sport_constants::{DEFAULT_LOSS_OF_DOWN_COUNT, MAX_CALL_TO_ACTIONS_PER_SERIES};

pub fn apply_loss_of_down(state: &mut MatchState, magnitude: Option<i32>) {
    let count = magnitude.unwrap_or(DEFAULT_LOSS_OF_DOWN_COUNT).max(1) as u8;
    let current_down = state.possession().down();
    let new_down = current_down + count;
    let max_downs = MAX_CALL_TO_ACTIONS_PER_SERIES as u8;

    if new_down > max_downs {
        let scrimmage = state.possession().scrimmage_point();
        let swapped_role = state.possession().role().swap();
        let mut new_series = state.possession().series_state().clone();
        new_series.reset(scrimmage);
        *state.possession_mut() = PossessionSnapshot::with_live_sequence(
            state.possession().ball_state(),
            state.possession().clock_state(),
            swapped_role,
            new_series,
            state.possession().live_sequence().clone(),
        );
    } else {
        for _ in 0..count {
            state.possession_mut().series_state_mut().advance_down();
        }
    }
}