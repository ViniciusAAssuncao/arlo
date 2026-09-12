use arlo_stats::PlayerDriveStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchPlayerDrivesRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub total_drives: i32,
    pub central_drives: i32,
    pub left_lateral_drives: i32,
    pub right_lateral_drives: i32,
    pub lateral_drives: i32,
    pub max_drives_in_series: i32,
}

impl MatchPlayerDrivesRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        total_drives: u32,
        central_drives: u32,
        left_lateral_drives: u32,
        right_lateral_drives: u32,
        lateral_drives: u32,
        max_drives_in_series: u32,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            total_drives: total_drives as i32,
            central_drives: central_drives as i32,
            left_lateral_drives: left_lateral_drives as i32,
            right_lateral_drives: right_lateral_drives as i32,
            lateral_drives: lateral_drives as i32,
            max_drives_in_series: max_drives_in_series as i32,
        }
    }

    pub fn from_stats(id: Uuid, match_id: Uuid, stats: &PlayerDriveStats) -> Self {
        Self::new(
            id,
            match_id,
            stats.player_id,
            stats.total_drives,
            stats.central_drives,
            stats.left_lateral_drives,
            stats.right_lateral_drives,
            stats.lateral_drives(),
            stats.max_drives_in_series,
        )
    }
}
