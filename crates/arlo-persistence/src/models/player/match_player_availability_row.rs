use arlo_stats::PlayerAvailabilityStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchPlayerAvailabilityRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub total_suspended_seconds: f64,
    pub expulsion_count: i32,
    pub is_currently_expelled: bool,
}

impl MatchPlayerAvailabilityRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        total_suspended_seconds: f64,
        expulsion_count: u32,
        is_currently_expelled: bool,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            total_suspended_seconds,
            expulsion_count: expulsion_count as i32,
            is_currently_expelled,
        }
    }

    pub fn from_stats(id: Uuid, match_id: Uuid, stats: &PlayerAvailabilityStats) -> Self {
        Self::new(
            id,
            match_id,
            stats.player_id,
            stats.total_suspended_seconds,
            stats.expulsion_count,
            stats.is_currently_expelled,
        )
    }
}
