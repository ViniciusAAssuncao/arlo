use arlo_events::{
    AvailabilityStatus, ImpulseEventKind, MatchEvent, MatchEventEnvelope, SubstitutionReason,
};
use uuid::Uuid;

pub fn filter_surviving_envelopes<'a>(
    first_sequence: u64,
    last_sequence: u64,
    envelopes: impl IntoIterator<Item = &'a MatchEventEnvelope>,
) -> Vec<&'a MatchEventEnvelope> {
    let mut preceding_injury: Option<Uuid> = None;
    envelopes
        .into_iter()
        .filter(|envelope| {
            let preserve_strain = matches!(
                envelope.event(),
                MatchEvent::PhysicalStrainRecorded(e) if preceding_injury == Some(e.player_id())
            );
            preceding_injury = match envelope.event() {
                MatchEvent::InjuryIncidentRecorded(e) => Some(e.player_id()),
                _ => None,
            };
            if envelope.sequence_number() < first_sequence
                || envelope.sequence_number() > last_sequence
            {
                return true;
            }
            preserve_strain
                || matches!(envelope.event(), MatchEvent::InjuryIncidentRecorded(_))
                || matches!(
                    envelope.event(),
                    MatchEvent::ImpulseShiftRecorded(e)
                        if e.event_kind() == ImpulseEventKind::InjurySetback
                )
                || matches!(
                    envelope.event(),
                    MatchEvent::PlayerAvailabilityChanged(e)
                        if e.new_status() == AvailabilityStatus::Injured
                )
                || matches!(
                    envelope.event(),
                    MatchEvent::SubstitutionMade(e)
                        if e.reason() == SubstitutionReason::Injury
                )
        })
        .collect()
}