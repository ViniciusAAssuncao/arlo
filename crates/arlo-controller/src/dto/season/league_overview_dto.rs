use super::fixture_summary_dto::FixtureSummaryDto;
use super::standings_entry_dto::StandingsEntryDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueOverviewDto {
    pub has_active_season: bool,
    pub standings: Vec<StandingsEntryDto>,
    pub fixtures: Vec<FixtureSummaryDto>,
    pub promotion_spots: u32,
    pub relegation_spots: u32,
    pub qualification_spots: u32,
    pub is_final_stage: bool,
}