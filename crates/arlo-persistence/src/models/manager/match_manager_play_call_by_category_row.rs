use arlo_events::PlayCallCategory;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchManagerPlayCallByCategoryRow {
    pub id: String,
    pub match_id: String,
    pub team_id: String,
    pub category: String,
    pub play_calls_count: i32,
}

impl MatchManagerPlayCallByCategoryRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        team_id: Uuid,
        category: impl Into<String>,
        play_calls_count: u32,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            team_id: team_id.to_string(),
            category: category.into(),
            play_calls_count: play_calls_count as i32,
        }
    }

    pub fn from_stats(
        id: Uuid,
        match_id: Uuid,
        team_id: Uuid,
        category: PlayCallCategory,
        count: u32,
    ) -> Self {
        Self::new(id, match_id, team_id, category.as_str(), count)
    }
}