use arlo_events::{MatchClockInstant, PlayCallSelected};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchPlayCallSelectionRow {
    pub id: String,
    pub match_id: String,
    pub sequence_number: i64,
    pub period: i32,
    pub seconds_in_period: f64,
    pub team_id: String,
    pub play_call_id: String,
    pub play_call_name: String,
    pub category: String,
}

impl MatchPlayCallSelectionRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        team_id: Uuid,
        play_call_id: Uuid,
        play_call_name: impl Into<String>,
        category: impl Into<String>,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            sequence_number: sequence_number as i64,
            period: period as i32,
            seconds_in_period,
            team_id: team_id.to_string(),
            play_call_id: play_call_id.to_string(),
            play_call_name: play_call_name.into(),
            category: category.into(),
        }
    }

    pub fn from_event(
        id: Uuid,
        match_id: Uuid,
        seq: u64,
        clock: MatchClockInstant,
        event: &PlayCallSelected,
    ) -> Self {
        Self::new(
            id,
            match_id,
            seq,
            clock.period(),
            clock.seconds_in_period(),
            event.team_id(),
            event.play_call_id(),
            event.play_call_name(),
            event.category().as_str(),
        )
    }
}
