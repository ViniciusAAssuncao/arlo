use arlo_events::{EventSink, InMemorySink, MatchEventEnvelope};
use arlo_stats::AggregatorRegistry;

pub struct DualEventSink<'a> {
    raw_sink: &'a mut InMemorySink,
    aggregator_registry: &'a mut AggregatorRegistry,
}

impl<'a> DualEventSink<'a> {
    pub fn new(
        raw_sink: &'a mut InMemorySink,
        aggregator_registry: &'a mut AggregatorRegistry,
    ) -> Self {
        Self {
            raw_sink,
            aggregator_registry,
        }
    }

    pub fn raw_sink(&self) -> &InMemorySink {
        self.raw_sink
    }

    pub fn raw_sink_mut(&mut self) -> &mut InMemorySink {
        self.raw_sink
    }

    pub fn aggregator_registry(&self) -> &AggregatorRegistry {
        self.aggregator_registry
    }

    pub fn aggregator_registry_mut(&mut self) -> &mut AggregatorRegistry {
        self.aggregator_registry
    }
}

impl<'a> EventSink for DualEventSink<'a> {
    fn record(&mut self, envelope: MatchEventEnvelope) {
        self.aggregator_registry.handle_envelope(&envelope);
        self.raw_sink.record(envelope);
    }
}