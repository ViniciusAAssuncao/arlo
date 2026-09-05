use arlo_events::{MatchEvent, MatchEventEnvelope};

pub trait StatAggregator: Send + Sync {
    fn handle_envelope(&mut self, envelope: &MatchEventEnvelope) {
        self.handle_event(envelope.event());
    }

    fn handle_event(&mut self, event: &MatchEvent);

    fn reset(&mut self);
}
