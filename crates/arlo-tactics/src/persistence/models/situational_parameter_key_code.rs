use crate::error::{TacticsError, TacticsResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SituationalParameterKey {
    DownPressure,
    DistanceUrgency,
    ScoringProximity,
    DriveScarcity,
    TargetingFlexibility,
}

pub fn parse_situational_parameter_key(code: &str) -> TacticsResult<SituationalParameterKey> {
    match code {
        "down_pressure" | "DownPressure" => Ok(SituationalParameterKey::DownPressure),
        "distance_urgency" | "DistanceUrgency" => Ok(SituationalParameterKey::DistanceUrgency),
        "scoring_proximity" | "ScoringProximity" => Ok(SituationalParameterKey::ScoringProximity),
        "drive_scarcity" | "DriveScarcity" => Ok(SituationalParameterKey::DriveScarcity),
        "targeting_flexibility" | "TargetingFlexibility" => {
            Ok(SituationalParameterKey::TargetingFlexibility)
        }
        _ => Err(TacticsError::InvalidInstructionKey(format!(
            "Invalid situational parameter key: {code}"
        ))),
    }
}

pub fn situational_parameter_key_to_code(key: SituationalParameterKey) -> &'static str {
    match key {
        SituationalParameterKey::DownPressure => "down_pressure",
        SituationalParameterKey::DistanceUrgency => "distance_urgency",
        SituationalParameterKey::ScoringProximity => "scoring_proximity",
        SituationalParameterKey::DriveScarcity => "drive_scarcity",
        SituationalParameterKey::TargetingFlexibility => "targeting_flexibility",
    }
}
