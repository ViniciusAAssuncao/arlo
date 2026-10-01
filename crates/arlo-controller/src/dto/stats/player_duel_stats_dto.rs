use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDuelKindStatsDto {
    pub duel_kind: String,
    pub total: u32,
    pub wins: u32,
    pub losses: u32,
    pub as_attacker_wins: u32,
    pub as_attacker_losses: u32,
    pub as_defender_wins: u32,
    pub as_defender_losses: u32,
    pub win_rate: f64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDuelStatsDto {
    pub total_duels: u32,
    pub total_wins: u32,
    pub total_losses: u32,
    pub win_rate: f64,
    pub attacker_duels: u32,
    pub attacker_wins: u32,
    pub attacker_losses: u32,
    pub attacker_win_rate: f64,
    pub defender_duels: u32,
    pub defender_wins: u32,
    pub defender_losses: u32,
    pub defender_win_rate: f64,
    pub by_kind: Vec<PlayerDuelKindStatsDto>,
    pub saves_attempted: u32,
    pub saves_made: u32,
    pub save_percentage: f64,
}
