use arlo_events::{ImpulseCriticalReached, MatchClockInstant};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchImpulseCriticalEventRow {
    pub id: String,
    pub match_id: String,
    pub sequence_number: i64,
    pub period: i32,
    pub seconds_in_period: f64,
    pub player_id: String,
    pub value: i32,
    pub duration_seconds: f64,
}

impl MatchImpulseCriticalEventRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        player_id: Uuid,
        value: u8,
        duration_seconds: f64,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            sequence_number: sequence_number as i64,
            period: period as i32,
            seconds_in_period,
            player_id: player_id.to_string(),
            value: value as i32,
            duration_seconds,
        }
    }

    pub fn from_event(
        id: Uuid,
        match_id: Uuid,
        seq: u64,
        clock: MatchClockInstant,
        event: &ImpulseCriticalReached,
    ) -> Self {
        Self::new(
            id,
            match_id,
            seq,
            clock.period(),
            clock.seconds_in_period(),
            event.player_id(),
            event.value(),
            event.duration_seconds(),
        )
    }
}
