use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAppearanceStatsDto {
    pub squad_selections: u32,
    pub starts: u32,
    pub appearances: u32,
    pub substitute_appearances: u32,
}
