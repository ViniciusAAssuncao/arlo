use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMatchPunishmentDto {
    pub yardage_loss_count: u32,
    pub loss_of_down_count: u32,
    pub loss_of_drive_count: u32,
    pub time_penalty_count: u32,
    pub expulsion_count: u32,
    pub invalidate_play_count: u32,
    pub total_yardage_loss_mirim: f64,
    pub total_loss_of_down_count: u32,
    pub total_time_penalty_seconds: f64,
    pub total_loss_of_drive_count: u32,
}