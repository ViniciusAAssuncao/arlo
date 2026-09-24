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
    PositionRange,
    ExternalCompetitionWinner,
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
        "PositionRange" | "position_range" => Ok(QualificationPoolKind::PositionRange),
        "ExternalCompetitionWinner" | "external_competition_winner" => {
            Ok(QualificationPoolKind::ExternalCompetitionWinner)
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
        QualificationPoolKind::PositionRange => "PositionRange",
        QualificationPoolKind::ExternalCompetitionWinner => "ExternalCompetitionWinner",
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualificationPoolRuleCodes {
    pub kind: &'static str,
    pub count: Option<i32>,
    pub position_index: Option<i32>,
    pub range_start_position: Option<i32>,
    pub range_end_position: Option<i32>,
    pub external_competition_id: Option<String>,
}

impl QualificationPoolRuleCodes {
    pub fn new(
        kind: &'static str,
        count: Option<i32>,
        position_index: Option<i32>,
        range_start_position: Option<i32>,
        range_end_position: Option<i32>,
        external_competition_id: Option<String>,
    ) -> Self {
        Self {
            kind,
            count,
            position_index,
            range_start_position,
            range_end_position,
            external_competition_id,
        }
    }
}

pub fn qualification_pool_rule_to_codes(
    rule: &QualificationPoolRule,
) -> QualificationPoolRuleCodes {
    match rule {
        QualificationPoolRule::AllTeams => {
            QualificationPoolRuleCodes::new("AllTeams", None, None, None, None, None)
        }
        QualificationPoolRule::TopN { count } => {
            QualificationPoolRuleCodes::new("TopN", Some(*count as i32), None, None, None, None)
        }
        QualificationPoolRule::BottomN { count } => {
            QualificationPoolRuleCodes::new("BottomN", Some(*count as i32), None, None, None, None)
        }
        QualificationPoolRule::GroupWinners => {
            QualificationPoolRuleCodes::new("GroupWinners", None, None, None, None, None)
        }
        QualificationPoolRule::GroupRunnersUp => {
            QualificationPoolRuleCodes::new("GroupRunnersUp", None, None, None, None, None)
        }
        QualificationPoolRule::BestAtGroupPosition {
            position_index,
            count,
        } => QualificationPoolRuleCodes::new(
            "BestAtGroupPosition",
            Some(*count as i32),
            Some(*position_index as i32),
            None,
            None,
            None,
        ),
        QualificationPoolRule::PositionRange {
            start_position,
            end_position,
        } => QualificationPoolRuleCodes::new(
            "PositionRange",
            None,
            None,
            Some(*start_position as i32),
            Some(*end_position as i32),
            None,
        ),
        QualificationPoolRule::ExternalCompetitionWinner { competition_id } => {
            QualificationPoolRuleCodes::new(
                "ExternalCompetitionWinner",
                None,
                None,
                None,
                None,
                Some(competition_id.to_string()),
            )
        }
    }
}
