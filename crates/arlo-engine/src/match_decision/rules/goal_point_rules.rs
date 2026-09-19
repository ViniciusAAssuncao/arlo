use crate::match_decision::rules::ScoringValidationError;
use arlo_domain::sport_constants::GOAL_POINT_REQUIRED_DRIVES;

pub fn can_attempt_goal_point(drives_in_series: u32) -> bool {
    drives_in_series >= GOAL_POINT_REQUIRED_DRIVES
}

pub fn validate_goal_point(
    drives_in_series: u32,
    is_bonus_phase: bool,
) -> Result<(), ScoringValidationError> {
    if is_bonus_phase {
        return Err(ScoringValidationError::FieldGoalOnlyAllowedInBonusPhase);
    }
    if !can_attempt_goal_point(drives_in_series) {
        return Err(ScoringValidationError::InsufficientDrivesForGoalPoint {
            current: drives_in_series,
            required: GOAL_POINT_REQUIRED_DRIVES,
        });
    }
    Ok(())
}