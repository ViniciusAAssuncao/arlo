use crate::possession::PossessionSnapshot;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;
use arlo_math::units::Position as VectorPosition;

pub fn apply_turnover_outcome(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    turnover_spot: VectorPosition,
) {
    let swapped_role = publisher.state().possession().role().swap();
    let mut new_series = publisher.state().possession().series_state().clone();
    new_series.reset(turnover_spot);

    let ball_state = publisher.state().possession().ball_state();
    let clock_state = publisher.state().possession().clock_state();
    let live_sequence = publisher.state().possession().live_sequence().clone();

    *publisher.state_mut().possession_mut() = PossessionSnapshot::with_live_sequence(
        ball_state,
        clock_state,
        swapped_role,
        new_series,
        live_sequence,
    );

    publisher.state_mut().reset_drives();
}