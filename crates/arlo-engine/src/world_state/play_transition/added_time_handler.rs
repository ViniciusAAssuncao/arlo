use crate::officiating::{is_added_time_eligible_period, AddedTimeDecisionEngine};
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;
use rand::Rng;

pub fn evaluate_and_apply_added_time<R: Rng + ?Sized>(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    rng: &mut R,
) -> bool {
    let period = publisher.state().clock().period();
    let format_rules = *publisher.state().format_rules();

    if !is_added_time_eligible_period(period, &format_rules) {
        return true;
    }

    if publisher.state().clock().is_added_time_decided() {
        return true;
    }

    let log = *publisher.state().added_time_tracker().current_log();
    let referee_table = publisher.state().head_referee_attribute_table();

    let duration =
        AddedTimeDecisionEngine::evaluate(&log, &referee_table, period, &format_rules, rng);

    publisher
        .state_mut()
        .clock_mut()
        .apply_added_time(duration.value());
    publisher
        .state_mut()
        .clock_mut()
        .mark_added_time_decided();
    publisher
        .state_mut()
        .mark_added_time_awarded(duration.value());

    publisher.emit_added_time_awarded(period, duration.value(), &log);

    let seconds_in_period = publisher.state().clock().seconds_in_period();
    let period_duration_seconds = publisher.state().clock().period_duration_seconds();

    seconds_in_period >= period_duration_seconds
}
