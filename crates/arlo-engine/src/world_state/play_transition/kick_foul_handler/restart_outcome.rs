use crate::world_state::play_transition::kick_foul_handler::possession_replacement::replace_possession_preserving_ball_and_clock;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;

pub fn apply_restart_outcome(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    reception_x_mirim: f64,
) {
    let current_role = *publisher.state().possession().role();
    let mut new_series = publisher.state().possession().series_state().clone();
    new_series.reset(reception_x_mirim);

    let new_origin = publisher.state().possession().possession_origin().clone();

    replace_possession_preserving_ball_and_clock(publisher.state_mut(), current_role, new_series, new_origin);
}