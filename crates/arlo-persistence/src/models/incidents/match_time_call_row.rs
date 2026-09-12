use arlo_events::{MatchClockInstant, TimeCallUsed};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchTimeCallRow {
    pub id: String,
    pub match_id: String,
    pub sequence_number: i64,
    pub period: i32,
    pub seconds_in_period: f64,
    pub team_id: String,
    pub remaining_time_calls_after: i32,
    pub reason: String,
}

impl MatchTimeCallRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        team_id: Uuid,
        remaining_time_calls_after: u32,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            sequence_number: sequence_number as i64,
            period: period as i32,
            seconds_in_period,
            team_id: team_id.to_string(),
            remaining_time_calls_after: remaining_time_calls_after as i32,
            reason: reason.into(),
        }
    }

    pub fn from_event(
        id: Uuid,
        match_id: Uuid,
        seq: u64,
        clock: MatchClockInstant,
        event: &TimeCallUsed,
    ) -> Self {
        Self::new(
            id,
            match_id,
            seq,
            clock.period(),
            clock.seconds_in_period(),
            event.team_id(),
            event.remaining_time_calls_after(),
            event.reason().as_str(),
        )
    }
}