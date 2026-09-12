use arlo_events::ImpulseEventKind;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchPlayerImpulseShiftByKindRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub event_kind: String,
    pub shifts_count: i32,
}

impl MatchPlayerImpulseShiftByKindRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        event_kind: impl Into<String>,
        shifts_count: u32,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            event_kind: event_kind.into(),
            shifts_count: shifts_count as i32,
        }
    }

    pub fn from_stats(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        kind: ImpulseEventKind,
        count: u32,
    ) -> Self {
        Self::new(id, match_id, player_id, kind.as_str(), count)
    }
}
