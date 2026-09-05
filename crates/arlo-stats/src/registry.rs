use crate::aggregator::StatAggregator;
use arlo_events::{MatchEvent, MatchEventEnvelope};

#[derive(Default)]
pub struct AggregatorRegistry {
    aggregators: Vec<Box<dyn StatAggregator>>,
}

impl AggregatorRegistry {
    pub fn new() -> Self {
        Self {
            aggregators: Vec::new(),
        }
    }

    pub fn register(&mut self, aggregator: Box<dyn StatAggregator>) {
        self.aggregators.push(aggregator);
    }

    pub fn register_aggregator<T: StatAggregator + 'static>(&mut self, aggregator: T) {
        self.aggregators.push(Box::new(aggregator));
    }

    pub fn handle_envelope(&mut self, envelope: &MatchEventEnvelope) {
        for aggregator in &mut self.aggregators {
            aggregator.handle_envelope(envelope);
        }
    }

    pub fn handle_event(&mut self, event: &MatchEvent) {
        for aggregator in &mut self.aggregators {
            aggregator.handle_event(event);
        }
    }

    pub fn handle_envelopes<'a>(
        &mut self,
        envelopes: impl IntoIterator<Item = &'a MatchEventEnvelope>,
    ) {
        for envelope in envelopes {
            self.handle_envelope(envelope);
        }
    }

    pub fn reset_all(&mut self) {
        for aggregator in &mut self.aggregators {
            aggregator.reset();
        }
    }

    pub fn aggregators(&self) -> &[Box<dyn StatAggregator>] {
        &self.aggregators
    }

    pub fn len(&self) -> usize {
        self.aggregators.len()
    }

    pub fn is_empty(&self) -> bool {
        self.aggregators.is_empty()
    }
}