use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerFoulOriginStatsDto {
    pub origin: String,
    pub count: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerFoulStatsDto {
    pub fouls_committed: u32,
    pub fouls_drawn: u32,
    pub correct_calls_committed: u32,
    pub incorrect_calls_committed: u32,
    pub expulsions: u32,
    pub time_penalties: u32,
    #[serde(default)]
    pub by_origin: Vec<PlayerFoulOriginStatsDto>,
}
