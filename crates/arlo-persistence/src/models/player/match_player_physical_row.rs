use arlo_stats::PlayerPhysicalStats;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct MatchPlayerPhysicalRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub end_energy_level: f64,
    pub peak_anaerobic_depletion: f64,
    pub total_distance_covered: f64,
    pub intra_match_recovery_amount: f64,
}

impl MatchPlayerPhysicalRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        end_energy_level: f64,
        peak_anaerobic_depletion: f64,
        total_distance_covered: f64,
        intra_match_recovery_amount: f64,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            end_energy_level,
            peak_anaerobic_depletion,
            total_distance_covered,
            intra_match_recovery_amount,
        }
    }

    pub fn from_stats(id: Uuid, match_id: Uuid, stats: &PlayerPhysicalStats) -> Self {
        Self::new(
            id,
            match_id,
            stats.player_id,
            stats.end_energy_level,
            stats.peak_anaerobic_depletion,
            stats.total_distance_covered,
            stats.intra_match_recovery_amount,
        )
    }
}
