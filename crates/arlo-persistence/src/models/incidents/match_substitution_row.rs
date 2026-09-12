use arlo_events::{MatchClockInstant, SubstitutionMade};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchSubstitutionRow {
    pub id: String,
    pub match_id: String,
    pub sequence_number: i64,
    pub period: i32,
    pub seconds_in_period: f64,
    pub team_id: String,
    pub player_out_id: String,
    pub player_in_id: String,
    pub reason: String,
}

impl MatchSubstitutionRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        team_id: Uuid,
        player_out_id: Uuid,
        player_in_id: Uuid,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            sequence_number: sequence_number as i64,
            period: period as i32,
            seconds_in_period,
            team_id: team_id.to_string(),
            player_out_id: player_out_id.to_string(),
            player_in_id: player_in_id.to_string(),
            reason: reason.into(),
        }
    }

    pub fn from_event(
        id: Uuid,
        match_id: Uuid,
        seq: u64,
        clock: MatchClockInstant,
        event: &SubstitutionMade,
    ) -> Self {
        Self::new(
            id,
            match_id,
            seq,
            clock.period(),
            clock.seconds_in_period(),
            event.team_id(),
            event.player_out(),
            event.player_in(),
            event.reason().as_str(),
        )
    }
}
