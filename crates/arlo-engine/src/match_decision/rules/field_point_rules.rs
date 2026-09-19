use crate::match_decision::rules::ScoringValidationError;
use arlo_domain::sport_constants::{
    FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM, FIELD_POINT_REQUIRED_DRIVES,
};

pub fn can_attempt_field_point(
    drives_in_series: u32,
    territory_advance_mirim: f64,
    normalized_proximity: f64,
) -> bool {
    drives_in_series >= FIELD_POINT_REQUIRED_DRIVES
        && (territory_advance_mirim >= FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM
            || normalized_proximity >= 0.70)
}

pub fn validate_field_point(
    drives_in_series: u32,
    territory_advance_mirim: f64,
    normalized_proximity: f64,
    is_bonus_phase: bool,
) -> Result<(), ScoringValidationError> {
    if is_bonus_phase {
        return Err(ScoringValidationError::FieldGoalOnlyAllowedInBonusPhase);
    }
    if drives_in_series < FIELD_POINT_REQUIRED_DRIVES {
        return Err(ScoringValidationError::InsufficientDrivesForFieldPoint {
            current: drives_in_series,
            required: FIELD_POINT_REQUIRED_DRIVES,
        });
    }
    if territory_advance_mirim < FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM
        && normalized_proximity < 0.70
    {
        return Err(ScoringValidationError::InsufficientTerritoryAdvance {
            current_mirim: territory_advance_mirim.max(0.0) as u32,
            required_mirim: FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM as u32,
        });
    }
    Ok(())
}