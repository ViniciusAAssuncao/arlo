use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerTouchStatsDto {
    pub total_touches: u32,
    pub passes_attempted: u32,
    pub passes_received: u32,
    pub drives_recorded: u32,
    pub recoveries: u32,
    pub scoring_attempts: u32,
    pub turnovers_conceded: u32,
}
