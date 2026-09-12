use arlo_stats::PlayerFoulStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchPlayerFoulRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub fouls_committed: i32,
    pub fouls_drawn: i32,
    pub correct_calls_committed: i32,
    pub incorrect_calls_committed: i32,
}

impl MatchPlayerFoulRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        fouls_committed: u32,
        fouls_drawn: u32,
        correct_calls_committed: u32,
        incorrect_calls_committed: u32,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            fouls_committed: fouls_committed as i32,
            fouls_drawn: fouls_drawn as i32,
            correct_calls_committed: correct_calls_committed as i32,
            incorrect_calls_committed: incorrect_calls_committed as i32,
        }
    }

    pub fn from_stats(id: Uuid, match_id: Uuid, stats: &PlayerFoulStats) -> Self {
        Self::new(
            id,
            match_id,
            stats.player_id,
            stats.fouls_committed,
            stats.fouls_drawn,
            stats.correct_calls_committed,
            stats.incorrect_calls_committed,
        )
    }
}
