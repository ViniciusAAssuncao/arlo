pub mod bonus_phase_rules;
pub mod field_point_rules;
pub mod goal_point_rules;

pub use bonus_phase_rules::*;
pub use field_point_rules::*;
pub use goal_point_rules::*;

use crate::match_decision::scoring::ScoringOpportunity;
use arlo_domain::sport_constants::{
    FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM, FIELD_POINT_REQUIRED_DRIVES,
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

pub fn validate_scoring_opportunity(
    is_bonus_phase: bool,
    drives_in_series: u32,
    territory_advance_mirim: f64,
    normalized_proximity: f64,
    finisher_rating: f64,
) -> Result<ScoringOpportunity, ScoringValidationError> {
    if is_bonus_phase {
        return validate_bonus_phase_field_goal(
            is_bonus_phase,
            finisher_rating,
            territory_advance_mirim,
        );
    }

    if validate_goal_point(drives_in_series, is_bonus_phase).is_ok() {
        return Ok(ScoringOpportunity::GoalPoint);
    }

    if validate_field_point(
        drives_in_series,
        territory_advance_mirim,
        normalized_proximity,
        is_bonus_phase,
    )
    .is_ok()
    {
        return Ok(ScoringOpportunity::FieldPoint);
    }

    if drives_in_series < FIELD_POINT_REQUIRED_DRIVES {
        return Err(ScoringValidationError::InsufficientDrivesForFieldPoint {
            current: drives_in_series,
            required: FIELD_POINT_REQUIRED_DRIVES,
        });
    }

    Err(ScoringValidationError::InsufficientTerritoryAdvance {
        current_mirim: territory_advance_mirim.max(0.0) as u32,
        required_mirim: FIELD_POINT_MIN_TERRITORY_ADVANCE_MIRIM as u32,
    })
}

pub fn evaluate_scoring_opportunity(
    is_bonus_phase: bool,
    drives_in_series: u32,
    territory_advance_mirim: f64,
    normalized_proximity: f64,
    finisher_rating: f64,
) -> ScoringOpportunity {
    validate_scoring_opportunity(
        is_bonus_phase,
        drives_in_series,
        territory_advance_mirim,
        normalized_proximity,
        finisher_rating,
    )
    .unwrap_or(ScoringOpportunity::None)
}