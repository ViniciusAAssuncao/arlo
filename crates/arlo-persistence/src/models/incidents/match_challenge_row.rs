use arlo_events::{ChallengeResolved, MatchClockInstant};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchChallengeRow {
    pub id: String,
    pub match_id: String,
    pub sequence_number: i64,
    pub period: i32,
    pub seconds_in_period: f64,
    pub team_id: String,
    pub call_kind: String,
    pub success: bool,
    pub remaining_challenges_after: i32,
}

impl MatchChallengeRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        team_id: Uuid,
        call_kind: impl Into<String>,
        success: bool,
        remaining_challenges_after: u32,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            sequence_number: sequence_number as i64,
            period: period as i32,
            seconds_in_period,
            team_id: team_id.to_string(),
            call_kind: call_kind.into(),
            success,
            remaining_challenges_after: remaining_challenges_after as i32,
        }
    }

    pub fn from_event(
        id: Uuid,
        match_id: Uuid,
        seq: u64,
        clock: MatchClockInstant,
        event: &ChallengeResolved,
    ) -> Self {
        Self::new(
            id,
            match_id,
            seq,
            clock.period(),
            clock.seconds_in_period(),
            event.team_id(),
            event.call_kind().as_str(),
            event.success(),
            event.remaining_challenges_after(),
        )
    }
}
