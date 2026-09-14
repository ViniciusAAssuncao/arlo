use crate::error::{DbError, DbResult};
use arlo_domain::QualificationPoolRule;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualificationPoolKind {
    AllTeams,
    TopN,
    BottomN,
    GroupWinners,
    GroupRunnersUp,
    BestAtGroupPosition,
}

pub fn parse_qualification_pool_kind(code: &str) -> DbResult<QualificationPoolKind> {
    match code {
        "AllTeams" | "all_teams" => Ok(QualificationPoolKind::AllTeams),
        "TopN" | "top_n" => Ok(QualificationPoolKind::TopN),
        "BottomN" | "bottom_n" => Ok(QualificationPoolKind::BottomN),
        "GroupWinners" | "group_winners" => Ok(QualificationPoolKind::GroupWinners),
        "GroupRunnersUp" | "group_runners_up" => Ok(QualificationPoolKind::GroupRunnersUp),
        "BestAtGroupPosition" | "best_at_group_position" => {
            Ok(QualificationPoolKind::BestAtGroupPosition)
        }
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid qualification pool kind: {code}"
        ))),
    }
}

pub fn qualification_pool_kind_to_code(kind: QualificationPoolKind) -> &'static str {
    match kind {
        QualificationPoolKind::AllTeams => "AllTeams",
        QualificationPoolKind::TopN => "TopN",
        QualificationPoolKind::BottomN => "BottomN",
        QualificationPoolKind::GroupWinners => "GroupWinners",
        QualificationPoolKind::GroupRunnersUp => "GroupRunnersUp",
        QualificationPoolKind::BestAtGroupPosition => "BestAtGroupPosition",
    }
}

pub fn qualification_pool_rule_to_codes(
    rule: &QualificationPoolRule,
) -> (&'static str, Option<i32>, Option<i32>) {
    match rule {
        QualificationPoolRule::AllTeams => ("AllTeams", None, None),
        QualificationPoolRule::TopN { count } => ("TopN", Some(*count as i32), None),
        QualificationPoolRule::BottomN { count } => ("BottomN", Some(*count as i32), None),
        QualificationPoolRule::GroupWinners => ("GroupWinners", None, None),
        QualificationPoolRule::GroupRunnersUp => ("GroupRunnersUp", None, None),
        QualificationPoolRule::BestAtGroupPosition {
            position_index,
            count,
        } => (
            "BestAtGroupPosition",
            Some(*count as i32),
            Some(*position_index as i32),
        ),
    }
}
