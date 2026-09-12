use arlo_stats::TeamPossessionStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchTeamPossessionRow {
    pub id: String,
    pub match_id: String,
    pub team_id: String,
    pub total_possession_seconds: f64,
}

impl MatchTeamPossessionRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        team_id: Uuid,
        total_possession_seconds: f64,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            team_id: team_id.to_string(),
            total_possession_seconds,
        }
    }

    pub fn from_stats(id: Uuid, match_id: Uuid, stats: &TeamPossessionStats) -> Self {
        Self::new(id, match_id, stats.team_id, stats.total_possession_seconds)
    }
}