use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RosterEntryDto {
    pub player_id: String,
    pub name: String,
    pub squad_number: Option<i32>,
    pub position: String,
    pub age: u32,
    pub current_ability: Option<i32>,
    pub condition: f64,
    pub morale: f64,
    pub height_m: f64,
    pub days_since_last_match: Option<u32>,
}

impl RosterEntryDto {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        player_id: impl Into<String>,
        name: impl Into<String>,
        squad_number: Option<i32>,
        position: impl Into<String>,
        age: u32,
        current_ability: Option<i32>,
        condition: f64,
        morale: f64,
        height_m: f64,
        days_since_last_match: Option<u32>,
    ) -> Self {
        Self {
            player_id: player_id.into(),
            name: name.into(),
            squad_number,
            position: position.into(),
            age,
            current_ability,
            condition,
            morale,
            height_m,
            days_since_last_match,
        }
    }
}

pub type TeamRosterEntryDto = RosterEntryDto;