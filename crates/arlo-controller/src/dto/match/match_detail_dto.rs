use super::match_lineup_dto::MatchLineupDto;
use super::match_manager_summary_dto::MatchManagerSummaryDto;
use super::match_officiating_dto::MatchOfficiatingDto;
use super::match_summary_dto::MatchSummaryDto;
use super::match_team_stats_dto::MatchTeamStatsDto;
use super::match_timeline_event_dto::MatchTimelineEventDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchDetailDto {
    pub summary: MatchSummaryDto,
    pub home_lineup: MatchLineupDto,
    pub away_lineup: MatchLineupDto,
    pub timeline: Vec<MatchTimelineEventDto>,
    pub officiating: MatchOfficiatingDto,
    pub team_stats: MatchTeamStatsDto,
    pub manager_summary: MatchManagerSummaryDto,
}