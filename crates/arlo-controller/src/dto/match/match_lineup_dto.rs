use super::match_squad_slot_dto::MatchSquadSlotDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchLineupDto {
    pub team_id: String,
    pub team_name: String,
    pub formation_id: String,
    pub formation_name: String,
    pub manager_id: String,
    pub manager_name: String,
    pub starters: Vec<MatchSquadSlotDto>,
    pub bench: Vec<MatchSquadSlotDto>,
}