use crate::dto::player_match::player_match_availability_dto::PlayerMatchAvailabilityDto;
use crate::dto::player_match::player_match_context_dto::PlayerMatchContextDto;
use crate::dto::player_match::player_match_impulse_dto::PlayerMatchImpulseDto;
use crate::dto::player_match::player_match_injury_dto::PlayerMatchInjuryDto;
use crate::dto::player_match::player_match_physical_dto::PlayerMatchPhysicalDto;
use crate::dto::player_match::player_match_punishment_dto::PlayerMatchPunishmentDto;
use crate::dto::stats::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMatchTelemetryDto {
    pub match_id: String,
    pub player_id: String,
    pub player_name: String,
    pub squad_number: Option<i32>,
    pub position: String,
    pub context: PlayerMatchContextDto,
    pub touches: PlayerTouchStatsDto,
    pub drives: PlayerDriveStatsDto,
    pub duels: PlayerDuelStatsDto,
    pub receiving: PlayerReceivingStatsDto,
    pub scoring: PlayerScoringStatsDto,
    pub assists: PlayerAssistStatsDto,
    pub artrine_decisions: PlayerArtrineDecisionStatsDto,
    pub fouls: PlayerFoulStatsDto,
    pub kick_fouls: PlayerKickFoulStatsDto,
    pub impulse: Option<PlayerMatchImpulseDto>,
    pub physical: Option<PlayerMatchPhysicalDto>,
    pub availability: PlayerMatchAvailabilityDto,
    pub injuries: PlayerMatchInjuryDto,
    pub punishments: PlayerMatchPunishmentDto,
}
