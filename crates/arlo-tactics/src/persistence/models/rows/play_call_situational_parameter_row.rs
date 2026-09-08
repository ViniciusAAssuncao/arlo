use crate::persistence::models::situational_parameter_key_code::SituationalParameterKey;
use crate::playcall::situational::SituationalProfile;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct PlayCallSituationalParameterRow {
    pub id: String,
    pub play_call_id: String,
    pub parameter_key: String,
    pub value: f64,
}

pub fn situational_profile_from_pairs(
    pairs: &[(SituationalParameterKey, f64)],
) -> SituationalProfile {
    let mut down_pressure = 0.0;
    let mut distance_urgency = 0.0;
    let mut scoring_proximity = 0.0;
    let mut drive_scarcity = 0.0;
    let mut targeting_flexibility = 0.0;

    for (key, val) in pairs {
        match key {
            SituationalParameterKey::DownPressure => down_pressure = *val,
            SituationalParameterKey::DistanceUrgency => distance_urgency = *val,
            SituationalParameterKey::ScoringProximity => scoring_proximity = *val,
            SituationalParameterKey::DriveScarcity => drive_scarcity = *val,
            SituationalParameterKey::TargetingFlexibility => targeting_flexibility = *val,
        }
    }

    SituationalProfile::new_clamped(
        down_pressure,
        distance_urgency,
        scoring_proximity,
        drive_scarcity,
        targeting_flexibility,
    )
}

pub fn situational_profile_to_pairs(
    profile: &SituationalProfile,
) -> Vec<(SituationalParameterKey, f64)> {
    vec![
        (
            SituationalParameterKey::DownPressure,
            profile.down_pressure().value(),
        ),
        (
            SituationalParameterKey::DistanceUrgency,
            profile.distance_urgency().value(),
        ),
        (
            SituationalParameterKey::ScoringProximity,
            profile.scoring_proximity().value(),
        ),
        (
            SituationalParameterKey::DriveScarcity,
            profile.drive_scarcity().value(),
        ),
        (
            SituationalParameterKey::TargetingFlexibility,
            profile.targeting_flexibility().value(),
        ),
    ]
}
