use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerKickFoulDecisionStatsDto {
    pub decision_kind: String,
    pub takes_count: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerKickFoulStatsDto {
    pub kick_foul_takes: u32,
    pub by_decision: Vec<PlayerKickFoulDecisionStatsDto>,
}
