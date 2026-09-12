use arlo_stats::PlayerTouchStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchPlayerTouchesRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub passes_attempted: i32,
    pub passes_received: i32,
    pub drives_recorded: i32,
    pub recoveries: i32,
    pub scoring_attempts: i32,
    pub total_touches: i32,
    pub turnovers_conceded: i32,
}

impl MatchPlayerTouchesRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        passes_attempted: u32,
        passes_received: u32,
        drives_recorded: u32,
        recoveries: u32,
        scoring_attempts: u32,
        total_touches: u32,
        turnovers_conceded: u32,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            passes_attempted: passes_attempted as i32,
            passes_received: passes_received as i32,
            drives_recorded: drives_recorded as i32,
            recoveries: recoveries as i32,
            scoring_attempts: scoring_attempts as i32,
            total_touches: total_touches as i32,
            turnovers_conceded: turnovers_conceded as i32,
        }
    }

    pub fn from_stats(id: Uuid, match_id: Uuid, stats: &PlayerTouchStats) -> Self {
        Self::new(
            id,
            match_id,
            stats.player_id,
            stats.passes_attempted,
            stats.passes_received,
            stats.drives_recorded,
            stats.recoveries,
            stats.scoring_attempts,
            stats.total_touches,
            stats.turnovers_conceded,
        )
    }
}
