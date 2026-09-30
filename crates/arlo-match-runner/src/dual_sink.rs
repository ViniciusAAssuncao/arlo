use arlo_events::{AvailabilityStatus, EventSink, ImpulseEventKind, InMemorySink, MatchEvent, MatchEventEnvelope, SubstitutionReason};
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

    pub fn invalidate_play(&mut self, first_sequence: u64, last_sequence: u64) {
        let mut preceding_injury = None;
        self.raw_sink.retain(|envelope| {
            let preserve_strain = matches!(envelope.event(), MatchEvent::PhysicalStrainRecorded(event)
                if preceding_injury == Some(event.player_id()));
            preceding_injury = match envelope.event() {
                MatchEvent::InjuryIncidentRecorded(event) => Some(event.player_id()),
                _ => None,
            };
            if envelope.sequence_number() < first_sequence || envelope.sequence_number() > last_sequence {
                return true;
            }
            preserve_strain || matches!(envelope.event(), MatchEvent::InjuryIncidentRecorded(_))
                || matches!(envelope.event(), MatchEvent::ImpulseShiftRecorded(event)
                    if event.event_kind() == ImpulseEventKind::InjurySetback)
                || matches!(envelope.event(),
                MatchEvent::PlayerAvailabilityChanged(event) if event.new_status() == AvailabilityStatus::Injured
            ) || matches!(envelope.event(),
                MatchEvent::SubstitutionMade(event) if event.reason() == SubstitutionReason::Injury
            )
        });
        self.aggregator_registry.reset_all();
        self.aggregator_registry.handle_envelopes(self.raw_sink.events());
    }
}

impl<'a> EventSink for DualEventSink<'a> {
    fn record(&mut self, envelope: MatchEventEnvelope) {
        self.aggregator_registry.handle_envelope(&envelope);
        self.raw_sink.record(envelope);
    }
}
