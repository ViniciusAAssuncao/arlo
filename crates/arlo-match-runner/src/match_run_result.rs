use arlo_events::InMemorySink;
use arlo_stats::AggregatorRegistry;

pub struct MatchRunResult {
    pub raw_sink: InMemorySink,
    pub aggregators: AggregatorRegistry,
}

impl MatchRunResult {
    pub fn new(raw_sink: InMemorySink, aggregators: AggregatorRegistry) -> Self {
        Self {
            raw_sink,
            aggregators,
        }
    }

    pub fn raw_sink(&self) -> &InMemorySink {
        &self.raw_sink
    }

    pub fn raw_sink_mut(&mut self) -> &mut InMemorySink {
        &mut self.raw_sink
    }

    pub fn aggregators(&self) -> &AggregatorRegistry {
        &self.aggregators
    }

    pub fn aggregators_mut(&mut self) -> &mut AggregatorRegistry {
        &mut self.aggregators
    }

    pub fn into_parts(self) -> (InMemorySink, AggregatorRegistry) {
        (self.raw_sink, self.aggregators)
    }
}