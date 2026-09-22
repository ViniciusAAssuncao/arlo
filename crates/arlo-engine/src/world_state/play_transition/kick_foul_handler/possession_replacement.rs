use crate::possession::{PossessionOrigin, PossessionRole, PossessionSnapshot, SeriesState};
use crate::world_state::match_state::MatchState;

pub fn replace_possession_preserving_ball_and_clock(
    state: &mut MatchState,
    role: PossessionRole,
    series: SeriesState,
    origin: PossessionOrigin,
) {
    let ball_state = state.possession().ball_state();
    let clock_state = state.possession().clock_state();
    let live_sequence = state.possession().live_sequence().clone();

    *state.possession_mut() = PossessionSnapshot::with_live_sequence(
        ball_state,
        clock_state,
        role,
        series,
        live_sequence,
        origin,
    ).with_current_carrier(None);

    state.reset_drives();
}