use crate::dto::calendar::ResolvedCalendarDateDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayAdvancementSummaryDto {
    pub resolved_date: ResolvedCalendarDateDto,
    pub season_generation_count: u32,
    pub stage_transition_count: u32,
    pub conflict_scan_count: u32,
    pub matches_played_count: u32,
}