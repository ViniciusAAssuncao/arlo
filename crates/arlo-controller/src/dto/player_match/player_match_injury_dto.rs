use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMatchInjuryByBodyRegionDto {
    pub body_region: String,
    pub injuries_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMatchInjuryDto {
    pub total_injuries: u32,
    pub contact_injuries: u32,
    pub non_contact_injuries: u32,
    pub grade_1_injuries: u32,
    pub grade_2_injuries: u32,
    pub grade_3_injuries: u32,
    pub by_body_region: Vec<PlayerMatchInjuryByBodyRegionDto>,
}