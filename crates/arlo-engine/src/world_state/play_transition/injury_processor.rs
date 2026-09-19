use crate::injury::outcome::InjuryIncidentResolution;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;

pub fn process_injuries<S: EventSink>(
    publisher: &mut EventPublisher<'_, S>,
    injuries: &[InjuryIncidentResolution],
) {
    for injury in injuries {
        publisher.emit_injury_incident(injury);
    }
}
