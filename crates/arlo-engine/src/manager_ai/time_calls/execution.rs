use crate::time::{DurationComponentKind, DurationLedger};
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;
use arlo_math::units::Duration;
use uuid::Uuid;

pub fn execute_time_call(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    is_home: bool,
    ledger: &mut DurationLedger,
) -> bool {
    let used = publisher.state_mut().clock_mut().use_time_call(is_home);
    if used {
        let duration_seconds = (publisher
            .state()
            .format_rules()
            .time_call_duration_minutes()
            * 60) as f64;
        let duration = Duration::new(duration_seconds);
        ledger.record_dead_ball(DurationComponentKind::Huddle, duration);

        let remaining_after = if is_home {
            publisher.state().clock().home_time_calls()
        } else {
            publisher.state().clock().away_time_calls()
        };

        publisher.emit_time_call_used(team_id, remaining_after);
        true
    } else {
        false
    }
}
