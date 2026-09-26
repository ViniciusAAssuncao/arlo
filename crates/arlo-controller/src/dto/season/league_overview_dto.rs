use super::fixture_summary_dto::FixtureSummaryDto;
use super::knockout_tie_dto::KnockoutTieDto;
use super::standings_entry_dto::StandingsEntryDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueOverviewDto {
    pub has_active_season: bool,
    pub current_stage_type: String,
    pub standings: Vec<StandingsEntryDto>,
    pub fixtures: Vec<FixtureSummaryDto>,
    pub knockout_ties: Vec<KnockoutTieDto>,
    pub promotion_spots: u32,
    pub relegation_spots: u32,
    pub qualification_spots: u32,
    pub is_final_stage: bool,
}