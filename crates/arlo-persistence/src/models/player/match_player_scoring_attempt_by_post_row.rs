use arlo_events::ScoringPost;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchPlayerScoringAttemptByPostRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub scoring_post: String,
    pub attempts: i32,
    pub converted: i32,
    pub missed: i32,
    pub conversion_rate: f64,
}

impl MatchPlayerScoringAttemptByPostRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        scoring_post: impl Into<String>,
        attempts: u32,
        converted: u32,
        missed: u32,
        conversion_rate: f64,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            scoring_post: scoring_post.into(),
            attempts: attempts as i32,
            converted: converted as i32,
            missed: missed as i32,
            conversion_rate,
        }
    }

    pub fn from_stats(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        post: ScoringPost,
        attempts: u32,
        converted: u32,
    ) -> Self {
        let missed = attempts.saturating_sub(converted);
        let conversion_rate = if attempts == 0 {
            0.0
        } else {
            (converted as f64) / (attempts as f64)
        };
        Self::new(
            id,
            match_id,
            player_id,
            post.as_str(),
            attempts,
            converted,
            missed,
            conversion_rate,
        )
    }
}
