use arlo_events::FoulOrigin;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchPlayerFoulByOriginRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub origin: String,
    pub fouls_count: i32,
}

impl MatchPlayerFoulByOriginRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        origin: impl Into<String>,
        fouls_count: u32,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            origin: origin.into(),
            fouls_count: fouls_count as i32,
        }
    }

    pub fn from_stats(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        origin: FoulOrigin,
        count: u32,
    ) -> Self {
        let origin_str = match origin {
            FoulOrigin::ContactDuel(kind) => format!("ContactDuel:{}", kind.as_str()),
            FoulOrigin::LineFault => "LineFault".to_string(),
        };
        Self::new(id, match_id, player_id, origin_str, count)
    }
}
