use arlo_events::{MatchClockInstant, TacticalProfileActivated};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchTacticalProfileActivationRow {
    pub id: String,
    pub match_id: String,
    pub sequence_number: i64,
    pub period: i32,
    pub seconds_in_period: f64,
    pub team_id: String,
    pub profile_id: String,
    pub profile_name: String,
}

impl MatchTacticalProfileActivationRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        team_id: Uuid,
        profile_id: Uuid,
        profile_name: impl Into<String>,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            sequence_number: sequence_number as i64,
            period: period as i32,
            seconds_in_period,
            team_id: team_id.to_string(),
            profile_id: profile_id.to_string(),
            profile_name: profile_name.into(),
        }
    }

    pub fn from_event(
        id: Uuid,
        match_id: Uuid,
        seq: u64,
        clock: MatchClockInstant,
        event: &TacticalProfileActivated,
    ) -> Self {
        Self::new(
            id,
            match_id,
            seq,
            clock.period(),
            clock.seconds_in_period(),
            event.team_id(),
            event.profile_id(),
            event.profile_name(),
        )
    }
}
