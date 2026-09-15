use crate::error::{DbError, DbResult};
use arlo_domain::LeagueMovementRule;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeagueMovementRuleKind {
    None,
    Automatic,
    PlayoffStage,
}

pub fn parse_league_movement_rule_kind(code: &str) -> DbResult<LeagueMovementRuleKind> {
    match code {
        "None" | "none" => Ok(LeagueMovementRuleKind::None),
        "Automatic" | "automatic" => Ok(LeagueMovementRuleKind::Automatic),
        "PlayoffStage" | "playoff_stage" => Ok(LeagueMovementRuleKind::PlayoffStage),
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid league movement rule kind: {code}"
        ))),
    }
}

pub fn league_movement_rule_kind_to_code(kind: LeagueMovementRuleKind) -> &'static str {
    match kind {
        LeagueMovementRuleKind::None => "None",
        LeagueMovementRuleKind::Automatic => "Automatic",
        LeagueMovementRuleKind::PlayoffStage => "PlayoffStage",
    }
}

pub fn league_movement_rule_to_codes(
    rule: &LeagueMovementRule,
) -> (&'static str, Option<i32>, Option<i32>) {
    match rule {
        LeagueMovementRule::None => ("None", None, None),
        LeagueMovementRule::Automatic { count } => ("Automatic", Some(*count as i32), None),
        LeagueMovementRule::PlayoffStage {
            stage_order_index,
            count,
        } => (
            "PlayoffStage",
            Some(*count as i32),
            Some(*stage_order_index as i32),
        ),
    }
}
