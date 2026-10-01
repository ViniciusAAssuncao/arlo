use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPositionDto {
    pub position: String,
    pub proficiency: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAttributeDto {
    pub key: String,
    pub name: String,
    pub value: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerProfileDto {
    pub id: String,
    pub name: String,
    pub birthdate_unix_seconds: i64,
    pub age: u32,
    pub height_m: f64,
    pub nationality_id: String,
    pub nationality_name: String,
    pub squad_number: Option<i32>,
    pub captaincy_role: Option<String>,
    pub team_id: Option<String>,
    pub team_name: Option<String>,
    pub positions: Vec<PlayerPositionDto>,
    pub current_ability: Option<i32>,
    pub attributes_by_category: HashMap<String, Vec<PlayerAttributeDto>>,
}
