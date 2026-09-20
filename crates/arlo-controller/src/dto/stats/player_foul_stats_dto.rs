use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerFoulStatsDto {
    pub fouls_committed: u32,
    pub fouls_drawn: u32,
    pub correct_calls_committed: u32,
    pub incorrect_calls_committed: u32,
    pub expulsions: u32,
    pub time_penalties: u32,
}
