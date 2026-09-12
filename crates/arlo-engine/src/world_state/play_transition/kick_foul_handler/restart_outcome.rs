use crate::world_state::play_transition::kick_foul_handler::possession_replacement::replace_possession_preserving_ball_and_clock;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;
use arlo_math::units::Position as VectorPosition;

pub fn apply_restart_outcome(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    reception_point: VectorPosition,
) {
    let current_role = *publisher.state().possession().role();
    let mut new_series = publisher.state().possession().series_state().clone();
    new_series.reset(reception_point);

    replace_possession_preserving_ball_and_clock(publisher.state_mut(), current_role, new_series);
}