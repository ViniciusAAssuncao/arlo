use crate::error::{TacticsError, TacticsResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlayerInstructionKey {
    PositioningBias,
    InvolvementPriority,
    CreativeLicense,
    EngagementBias,
    DepthDiscipline,
    TransitionUrgency,
    ReleaseTempo,
}

pub fn parse_player_instruction_key(code: &str) -> TacticsResult<PlayerInstructionKey> {
    match code {
        "positioning_bias" => Ok(PlayerInstructionKey::PositioningBias),
        "involvement_priority" => Ok(PlayerInstructionKey::InvolvementPriority),
        "creative_license" => Ok(PlayerInstructionKey::CreativeLicense),
        "engagement_bias" => Ok(PlayerInstructionKey::EngagementBias),
        "depth_discipline" => Ok(PlayerInstructionKey::DepthDiscipline),
        "transition_urgency" => Ok(PlayerInstructionKey::TransitionUrgency),
        "release_tempo" => Ok(PlayerInstructionKey::ReleaseTempo),
        _ => Err(TacticsError::InvalidInstructionKey(format!(
            "Invalid player instruction key: {code}"
        ))),
    }
}

pub fn player_instruction_key_to_code(key: PlayerInstructionKey) -> &'static str {
    match key {
        PlayerInstructionKey::PositioningBias => "positioning_bias",
        PlayerInstructionKey::InvolvementPriority => "involvement_priority",
        PlayerInstructionKey::CreativeLicense => "creative_license",
        PlayerInstructionKey::EngagementBias => "engagement_bias",
        PlayerInstructionKey::DepthDiscipline => "depth_discipline",
        PlayerInstructionKey::TransitionUrgency => "transition_urgency",
        PlayerInstructionKey::ReleaseTempo => "release_tempo",
    }
}