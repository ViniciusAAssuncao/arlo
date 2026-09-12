use arlo_stats::PlayerKickFoulStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchPlayerKickFoulRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub kick_foul_takes: i32,
}

impl MatchPlayerKickFoulRow {
    pub fn new(id: Uuid, match_id: Uuid, player_id: Uuid, kick_foul_takes: u32) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            kick_foul_takes: kick_foul_takes as i32,
        }
    }

    pub fn from_stats(id: Uuid, match_id: Uuid, stats: &PlayerKickFoulStats) -> Self {
        Self::new(id, match_id, stats.player_id, stats.kick_foul_takes)
    }
}
