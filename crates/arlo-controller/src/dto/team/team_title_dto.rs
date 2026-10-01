use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamTitleDto {
    pub id: String,
    pub competition_id: String,
    pub competition_name: String,
    pub competition_kind: String,
    pub season_label: String,
}
