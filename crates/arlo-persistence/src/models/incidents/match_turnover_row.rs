use arlo_events::{MatchClockInstant, Turnover};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchTurnoverRow {
    pub id: String,
    pub match_id: String,
    pub sequence_number: i64,
    pub period: i32,
    pub seconds_in_period: f64,
    pub previous_offense_team_id: String,
    pub new_offense_team_id: String,
    pub recovering_player_id: Option<String>,
    pub lost_by_player_id: Option<String>,
    pub in_live_play: bool,
    pub point_x: f64,
    pub point_y: f64,
}

impl MatchTurnoverRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        previous_offense_team_id: Uuid,
        new_offense_team_id: Uuid,
        recovering_player_id: Option<Uuid>,
        lost_by_player_id: Option<Uuid>,
        in_live_play: bool,
        point_x: f64,
        point_y: f64,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            sequence_number: sequence_number as i64,
            period: period as i32,
            seconds_in_period,
            previous_offense_team_id: previous_offense_team_id.to_string(),
            new_offense_team_id: new_offense_team_id.to_string(),
            recovering_player_id: recovering_player_id.map(|id| id.to_string()),
            lost_by_player_id: lost_by_player_id.map(|id| id.to_string()),
            in_live_play,
            point_x,
            point_y,
        }
    }

    pub fn from_event(
        id: Uuid,
        match_id: Uuid,
        seq: u64,
        clock: MatchClockInstant,
        event: &Turnover,
    ) -> Self {
        Self::new(
            id,
            match_id,
            seq,
            clock.period(),
            clock.seconds_in_period(),
            event.previous_offense(),
            event.new_offense(),
            event.recovering_player(),
            event.lost_by_player_id(),
            event.in_live_play(),
            event.point_x(),
            event.point_y(),
        )
    }
}
