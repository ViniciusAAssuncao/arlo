use crate::artrine::constants::FIELD_POINT_OPPORTUNITY_ADVANCE_BUFFER_MIRIM;
use crate::match_decision::ScoringOpportunity;
use crate::scoring_regime::policy::ScoringRegimePolicy;

pub fn evaluate_scoring_opportunity(
    policy: &ScoringRegimePolicy,
    is_bonus_phase: bool,
    drives_in_series: u32,
    territory_advance_mirim: f64,
    down: u8,
    normalized_proximity: f64,
) -> ScoringOpportunity {
    if is_bonus_phase {
        ScoringOpportunity::FieldGoal
    } else if drives_in_series >= policy.goal_point_required_drives {
        ScoringOpportunity::GoalPoint
    } else if drives_in_series >= policy.field_point_required_drives
        && territory_advance_mirim
            >= policy.field_point_min_territory_advance_mirim
                - FIELD_POINT_OPPORTUNITY_ADVANCE_BUFFER_MIRIM
    {
        ScoringOpportunity::FieldPoint
    } else if drives_in_series >= policy.field_point_required_drives
        && (down == 3 || down == 4)
        && normalized_proximity > 0.8
    {
        ScoringOpportunity::FieldPoint
    } else {
        ScoringOpportunity::None
    }
}

pub fn can_attempt_cross_or_finish(
    policy: &ScoringRegimePolicy,
    is_bonus_phase: bool,
    drives_in_series: u32,
    territory_advance_mirim: f64,
    buffer_mirim: f64,
    down: u8,
    normalized_proximity: f64,
) -> bool {
    if is_bonus_phase {
        true
    } else if drives_in_series >= policy.goal_point_required_drives {
        true
    } else if drives_in_series >= policy.field_point_required_drives
        && territory_advance_mirim >= policy.field_point_min_territory_advance_mirim - buffer_mirim
    {
        true
    } else if drives_in_series >= policy.field_point_required_drives
        && (down == 3 || down == 4)
        && normalized_proximity > 0.8
    {
        true
    } else {
        false
    }
}