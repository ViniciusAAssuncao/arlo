use crate::world_state::MatchState;
use arlo_tactics::{
    derive_distance_urgency, derive_down_pressure, derive_drive_scarcity, derive_scoring_proximity,
    SituationalContext,
};

pub fn build_situational_context(
    state: &MatchState,
    normalized_x_to_goal: f64,
) -> SituationalContext {
    let down_pressure = derive_down_pressure(state.possession().down() as u8);
    let distance_urgency =
        derive_distance_urgency(state.possession().series_state().remaining_mirins_to_target());
    let scoring_proximity = derive_scoring_proximity(normalized_x_to_goal);
    let drive_scarcity = derive_drive_scarcity(state.drives_in_current_series());

    SituationalContext::new(
        down_pressure,
        distance_urgency,
        scoring_proximity,
        drive_scarcity,
    )
}
