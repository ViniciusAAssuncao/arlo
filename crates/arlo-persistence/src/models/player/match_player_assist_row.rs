use arlo_stats::PlayerAssistsStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchPlayerAssistRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub goalpoint_assists: i32,
}

impl MatchPlayerAssistRow {
    pub fn new(id: Uuid, match_id: Uuid, player_id: Uuid, goalpoint_assists: u32) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            goalpoint_assists: goalpoint_assists as i32,
        }
    }

    pub fn from_stats(id: Uuid, match_id: Uuid, stats: &PlayerAssistsStats) -> Self {
        Self::new(id, match_id, stats.player_id, stats.goalpoint_assists)
    }
}
