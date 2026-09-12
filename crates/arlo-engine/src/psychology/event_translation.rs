use crate::psychology::systems::critical::ImpulseCriticalReached as EngineImpulseCritical;
use crate::psychology::systems::events::{
    ImpulseEvent, ImpulseEventKind as EngineImpulseEventKind, ImpulseShift,
};
use arlo_events::{
    ImpulseCriticalReached as PublicImpulseCriticalReached,
    ImpulseEventKind as PublicImpulseEventKind, ImpulseShiftRecorded, MatchClockInstant,
    MatchEvent, MatchEventEnvelope,
};
use uuid::Uuid;

pub fn translate_impulse_event_kind(kind: EngineImpulseEventKind) -> PublicImpulseEventKind {
    match kind {
        EngineImpulseEventKind::DuelWon => PublicImpulseEventKind::DuelWon,
        EngineImpulseEventKind::DuelLost => PublicImpulseEventKind::DuelLost,
        EngineImpulseEventKind::ScoreFor => PublicImpulseEventKind::ScoreFor,
        EngineImpulseEventKind::ScoreAgainst => PublicImpulseEventKind::ScoreAgainst,
        EngineImpulseEventKind::TurnoverCommitted => PublicImpulseEventKind::TurnoverCommitted,
        EngineImpulseEventKind::TurnoverWon => PublicImpulseEventKind::TurnoverWon,
        EngineImpulseEventKind::SeriesSuccess => PublicImpulseEventKind::SeriesSuccess,
        EngineImpulseEventKind::SeriesFailure => PublicImpulseEventKind::SeriesFailure,
        EngineImpulseEventKind::MilestoneStreak => PublicImpulseEventKind::MilestoneStreak,
        EngineImpulseEventKind::BigPlayCompleted => PublicImpulseEventKind::BigPlayCompleted,
        EngineImpulseEventKind::BigPlayAllowed => PublicImpulseEventKind::BigPlayAllowed,
        EngineImpulseEventKind::FoulCommitted => PublicImpulseEventKind::FoulCommitted,
        EngineImpulseEventKind::FoulDrawn => PublicImpulseEventKind::FoulDrawn,
    }
}

pub fn translate_impulse_shift_recorded(
    player_id: Uuid,
    shift: &ImpulseShift,
    event: &ImpulseEvent,
) -> ImpulseShiftRecorded {
    ImpulseShiftRecorded::new(
        player_id,
        shift.previous_value(),
        shift.new_value(),
        translate_impulse_event_kind(event.kind()),
        event.surprisal(),
    )
}

pub fn translate_impulse_critical_reached(
    critical: &EngineImpulseCritical,
) -> PublicImpulseCriticalReached {
    PublicImpulseCriticalReached::new(
        critical.player_id(),
        critical.impulse_value(),
        critical.duration_below_floor_seconds(),
    )
}

pub fn create_psychology_envelope(
    sequence_number: u64,
    clock: MatchClockInstant,
    event: impl Into<MatchEvent>,
) -> MatchEventEnvelope {
    MatchEventEnvelope::new(sequence_number, clock, event.into())
}
