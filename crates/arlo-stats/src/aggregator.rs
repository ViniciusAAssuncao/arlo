use arlo_events::{MatchEvent, MatchEventEnvelope};
use std::any::Any;

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait StatAggregator: AsAny + Send + Sync + 'static {
    fn handle_envelope(&mut self, envelope: &MatchEventEnvelope) {
        self.handle_event(envelope.event());
    }

    fn handle_event(&mut self, event: &MatchEvent);

    fn reset(&mut self);
}
