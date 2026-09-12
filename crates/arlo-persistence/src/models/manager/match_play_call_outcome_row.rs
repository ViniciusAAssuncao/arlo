use arlo_stats::PlayCallOutcomeStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchPlayCallOutcomeRow {
    pub id: String,
    pub match_id: String,
    pub play_call_id: String,
    pub attempts: i32,
    pub successes: i32,
}

impl MatchPlayCallOutcomeRow {
    pub fn new(id: Uuid, match_id: Uuid, play_call_id: Uuid, attempts: u32, successes: u32) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            play_call_id: play_call_id.to_string(),
            attempts: attempts as i32,
            successes: successes as i32,
        }
    }

    pub fn from_stats(id: Uuid, match_id: Uuid, stats: &PlayCallOutcomeStats) -> Self {
        Self::new(id, match_id, stats.play_call_id, stats.attempts, stats.successes)
    }
}