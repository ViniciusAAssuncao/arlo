use arlo_events::{AvailabilityStatus, MatchClockInstant, PlayerAvailabilityChanged};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchAvailabilityChangeRow {
    pub id: String,
    pub match_id: String,
    pub sequence_number: i64,
    pub period: i32,
    pub seconds_in_period: f64,
    pub player_id: String,
    pub team_id: String,
    pub previous_status: String,
    pub new_status: String,
    pub remaining_seconds: Option<f64>,
}

impl MatchAvailabilityChangeRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        player_id: Uuid,
        team_id: Uuid,
        previous_status: impl Into<String>,
        new_status: impl Into<String>,
        remaining_seconds: Option<f64>,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            sequence_number: sequence_number as i64,
            period: period as i32,
            seconds_in_period,
            player_id: player_id.to_string(),
            team_id: team_id.to_string(),
            previous_status: previous_status.into(),
            new_status: new_status.into(),
            remaining_seconds,
        }
    }

    pub fn from_event(
        id: Uuid,
        match_id: Uuid,
        seq: u64,
        clock: MatchClockInstant,
        event: &PlayerAvailabilityChanged,
    ) -> Self {
        let prev_str = match event.previous_status() {
            AvailabilityStatus::Active => "Active",
            AvailabilityStatus::Suspended => "Suspended",
            AvailabilityStatus::Expelled => "Expelled",
            AvailabilityStatus::Injured => "Injured",
        };

        let new_str = match event.new_status() {
            AvailabilityStatus::Active => "Active",
            AvailabilityStatus::Suspended => "Suspended",
            AvailabilityStatus::Expelled => "Expelled",
            AvailabilityStatus::Injured => "Injured",
        };

        Self::new(
            id,
            match_id,
            seq,
            clock.period(),
            clock.seconds_in_period(),
            event.player_id(),
            event.team_id(),
            prev_str,
            new_str,
            event.remaining_seconds(),
        )
    }
}
