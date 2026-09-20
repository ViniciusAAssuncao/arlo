use crate::dto::season::StandingsEntryDto;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamStandingsPositionDto {
    pub position: u32,
    pub total_teams: u32,
    #[serde(flatten)]
    pub entry: StandingsEntryDto,
}

impl Deref for TeamStandingsPositionDto {
    type Target = StandingsEntryDto;

    fn deref(&self) -> &Self::Target {
        &self.entry
    }
}