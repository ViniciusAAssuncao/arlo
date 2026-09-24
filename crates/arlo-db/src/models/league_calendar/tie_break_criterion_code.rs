use crate::error::{DbError, DbResult};
use arlo_domain::TieBreakCriterion;

pub fn parse_tie_break_criterion(code: &str) -> DbResult<TieBreakCriterion> {
    match code {
        "IspaTotal" | "ispa_total" => Ok(TieBreakCriterion::IspaTotal),
        "QtaScore" | "qta_score" => Ok(TieBreakCriterion::QtaScore),
        "GoalDifference" | "goal_difference" => Ok(TieBreakCriterion::GoalDifference),
        "GoalPointsTotal" | "goal_points_total" => Ok(TieBreakCriterion::GoalPointsTotal),
        "HeadToHead" | "head_to_head" => Ok(TieBreakCriterion::HeadToHead),
        "Random" | "random" => Ok(TieBreakCriterion::Random),
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid tie break criterion: {code}"
        ))),
    }
}

pub fn tie_break_criterion_to_code(criterion: TieBreakCriterion) -> &'static str {
    match criterion {
        TieBreakCriterion::IspaTotal => "IspaTotal",
        TieBreakCriterion::QtaScore => "QtaScore",
        TieBreakCriterion::GoalDifference => "GoalDifference",
        TieBreakCriterion::GoalPointsTotal => "GoalPointsTotal",
        TieBreakCriterion::HeadToHead => "HeadToHead",
        TieBreakCriterion::Random => "Random",
    }
}
