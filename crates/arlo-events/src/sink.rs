use crate::envelope::MatchEventEnvelope;

pub trait EventSink: Send {
    fn record(&mut self, envelope: MatchEventEnvelope);

    fn record_all(&mut self, envelopes: impl IntoIterator<Item = MatchEventEnvelope>) {
        for envelope in envelopes {
            self.record(envelope);
        }
    }
}

impl<S: EventSink + ?Sized> EventSink for &mut S {
    fn record(&mut self, envelope: MatchEventEnvelope) {
        (**self).record(envelope);
    }
}

impl<S: EventSink + ?Sized> EventSink for Box<S> {
    fn record(&mut self, envelope: MatchEventEnvelope) {
        (**self).record(envelope);
    }
}