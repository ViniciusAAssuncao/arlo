use crate::envelope::MatchEventEnvelope;
use crate::sink::EventSink;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct InMemorySink {
    events: Vec<MatchEventEnvelope>,
}

impl InMemorySink {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            events: Vec::with_capacity(capacity),
        }
    }

    pub fn events(&self) -> &[MatchEventEnvelope] {
        &self.events
    }

    pub fn into_events(self) -> Vec<MatchEventEnvelope> {
        self.events
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&MatchEventEnvelope> {
        self.events.get(index)
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

impl EventSink for InMemorySink {
    fn record(&mut self, envelope: MatchEventEnvelope) {
        self.events.push(envelope);
    }
}