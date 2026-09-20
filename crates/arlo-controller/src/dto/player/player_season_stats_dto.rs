use crate::dto::stats::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSeasonStatsDto {
    pub player_id: String,
    pub season_instance_id: Option<String>,
    pub competition_id: Option<String>,
    pub competition_name: Option<String>,
    pub season_label: Option<String>,
    pub primary_role: Option<String>,
    pub appearances: PlayerAppearanceStatsDto,
    pub scoring: PlayerScoringStatsDto,
    pub assists: PlayerAssistStatsDto,
    pub touches: PlayerTouchStatsDto,
    pub duels: PlayerDuelStatsDto,
    pub receiving: PlayerReceivingStatsDto,
    pub drives: PlayerDriveStatsDto,
    pub fouls: PlayerFoulStatsDto,
    pub kick_fouls: PlayerKickFoulStatsDto,
    pub artrine_decisions: PlayerArtrineDecisionStatsDto,
}
