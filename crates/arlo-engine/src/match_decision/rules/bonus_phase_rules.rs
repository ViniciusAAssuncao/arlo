use crate::match_decision::rules::ScoringValidationError;
use crate::match_decision::scoring::ScoringOpportunity;
use crate::set_piece::select_kick_post;

pub fn can_attempt_field_goal(is_bonus_phase: bool) -> bool {
    is_bonus_phase
}

pub fn validate_bonus_phase_field_goal(
    is_bonus_phase: bool,
    finisher_rating: f64,
    territory_advance_mirim: f64,
) -> Result<ScoringOpportunity, ScoringValidationError> {
    if !is_bonus_phase {
        return Err(ScoringValidationError::OpenPlayCannotAttemptFieldGoal);
    }
    let post = select_kick_post(finisher_rating, territory_advance_mirim);
    Ok(ScoringOpportunity::FieldGoal(post))
}