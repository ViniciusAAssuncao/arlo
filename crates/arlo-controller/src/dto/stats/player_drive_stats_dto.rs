use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDriveStatsDto {
    pub total_drives: u32,
    pub central_drives: u32,
    pub left_lateral_drives: u32,
    pub right_lateral_drives: u32,
    pub lateral_drives: u32,
    pub max_drives_in_series: u32,
}
