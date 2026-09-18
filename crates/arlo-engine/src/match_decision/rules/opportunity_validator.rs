use crate::match_decision::scoring::ScoringOpportunity;
use crate::set_piece::select_kick_post;
use arlo_domain::sport_constants::{
    FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM, FIELD_POINT_REQUIRED_DRIVES,
    GOAL_POINT_REQUIRED_DRIVES,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScoringValidationError {
    InsufficientDrivesForGoalPoint { current: u32, required: u32 },
    InsufficientDrivesForFieldPoint { current: u32, required: u32 },
    InsufficientTerritoryAdvance { current_mirim: u32, required_mirim: u32 },
    FieldGoalOnlyAllowedInBonusPhase,
    OpenPlayCannotAttemptFieldGoal,
    NoOpportunityCriteriaMet,
}

pub fn can_attempt_goal_point(drives_in_series: u32) -> bool {
    drives_in_series >= GOAL_POINT_REQUIRED_DRIVES
}

pub fn can_attempt_field_point(drives_in_series: u32, territory_advance_mirim: f64) -> bool {
    drives_in_series >= FIELD_POINT_REQUIRED_DRIVES
        && territory_advance_mirim >= FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM
}

pub fn can_attempt_field_goal(is_bonus_phase: bool) -> bool {
    is_bonus_phase
}

pub fn validate_scoring_opportunity(
    is_bonus_phase: bool,
    drives_in_series: u32,
    territory_advance_mirim: f64,
    finisher_rating: f64,
) -> Result<ScoringOpportunity, ScoringValidationError> {
    if is_bonus_phase {
        let post = select_kick_post(finisher_rating, territory_advance_mirim);
        return Ok(ScoringOpportunity::FieldGoal(post));
    }

    if can_attempt_goal_point(drives_in_series) {
        return Ok(ScoringOpportunity::GoalPoint);
    }

    if can_attempt_field_point(drives_in_series, territory_advance_mirim) {
        return Ok(ScoringOpportunity::FieldPoint);
    }

    if drives_in_series < FIELD_POINT_REQUIRED_DRIVES {
        return Err(ScoringValidationError::InsufficientDrivesForFieldPoint {
            current: drives_in_series,
            required: FIELD_POINT_REQUIRED_DRIVES,
        });
    }

    if territory_advance_mirim < FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM {
        return Err(ScoringValidationError::InsufficientTerritoryAdvance {
            current_mirim: territory_advance_mirim.max(0.0) as u32,
            required_mirim: FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM as u32,
        });
    }

    Err(ScoringValidationError::NoOpportunityCriteriaMet)
}

pub fn evaluate_scoring_opportunity(
    is_bonus_phase: bool,
    drives_in_series: u32,
    territory_advance_mirim: f64,
    finisher_rating: f64,
) -> ScoringOpportunity {
    validate_scoring_opportunity(
        is_bonus_phase,
        drives_in_series,
        territory_advance_mirim,
        finisher_rating,
    )
    .unwrap_or(ScoringOpportunity::None)
}