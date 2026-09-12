use arlo_stats::PlayerScoringAttemptStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchPlayerScoringAttemptRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub attempts: i32,
    pub converted: i32,
    pub missed: i32,
    pub conversion_rate: f64,
    pub miss_rate: f64,
    pub goal_points_scored: i32,
    pub field_points_scored: i32,
    pub field_goals_scored: i32,
    pub total_points_scored: i32,
}

impl MatchPlayerScoringAttemptRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        attempts: u32,
        converted: u32,
        missed: u32,
        conversion_rate: f64,
        miss_rate: f64,
        goal_points_scored: u32,
        field_points_scored: u32,
        field_goals_scored: u32,
        total_points_scored: u32,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            attempts: attempts as i32,
            converted: converted as i32,
            missed: missed as i32,
            conversion_rate,
            miss_rate,
            goal_points_scored: goal_points_scored as i32,
            field_points_scored: field_points_scored as i32,
            field_goals_scored: field_goals_scored as i32,
            total_points_scored: total_points_scored as i32,
        }
    }

    pub fn from_stats(id: Uuid, match_id: Uuid, stats: &PlayerScoringAttemptStats) -> Self {
        Self::new(
            id,
            match_id,
            stats.player_id,
            stats.attempts,
            stats.converted,
            stats.missed,
            stats.conversion_rate(),
            stats.miss_rate(),
            stats.goal_points_scored,
            stats.field_points_scored,
            stats.field_goals_scored,
            stats.total_points_scored,
        )
    }
}
