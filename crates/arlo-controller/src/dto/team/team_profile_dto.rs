use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamVenueDto {
    pub id: String,
    pub name: String,
    pub capacity: Option<i32>,
    pub pitch_length_mirim: Option<f64>,
    pub pitch_width_mirim: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamProfileDto {
    pub id: String,
    pub name: String,
    pub country_id: String,
    pub country_name: String,
    pub prestige: i32,
    pub founded_at_unix_seconds: i64,
    pub primary_color_hex: Option<String>,
    pub secondary_color_hex: Option<String>,
    pub venue: Option<TeamVenueDto>,
    pub manager_id: Option<String>,
    pub manager_name: Option<String>,
    pub league_id: Option<String>,
    pub league_name: Option<String>,
    pub division_index: Option<u32>,
}
