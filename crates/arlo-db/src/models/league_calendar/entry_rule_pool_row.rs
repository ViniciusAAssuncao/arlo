use crate::error::{ DbError, DbResult };
use crate::models::league_calendar::entry_rule_pool_codes::{
    parse_qualification_pool_kind,
    QualificationPoolKind,
};
use arlo_domain::QualificationPoolRule;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct EntryRulePoolRow {
    pub id: String,
    pub league_calendar_stage_definition_id: String,
    pub pool_order_index: i32,
    pub pool_kind: String,
    pub count: Option<i32>,
    pub position_index: Option<i32>,
}

impl EntryRulePoolRow {
    pub fn to_domain(&self) -> DbResult<QualificationPoolRule> {
        let pool_kind = parse_qualification_pool_kind(&self.pool_kind)?;
        match pool_kind {
            QualificationPoolKind::AllTeams => Ok(QualificationPoolRule::AllTeams),
            QualificationPoolKind::TopN => {
                let count = self.count.ok_or_else(|| {
                    DbError::InvalidData("TopN pool rule requires count".to_string())
                })?;
                Ok(QualificationPoolRule::TopN {
                    count: count as u32,
                })
            }
            QualificationPoolKind::BottomN => {
                let count = self.count.ok_or_else(|| {
                    DbError::InvalidData("BottomN pool rule requires count".to_string())
                })?;
                Ok(QualificationPoolRule::BottomN {
                    count: count as u32,
                })
            }
            QualificationPoolKind::GroupWinners => Ok(QualificationPoolRule::GroupWinners),
            QualificationPoolKind::GroupRunnersUp => Ok(QualificationPoolRule::GroupRunnersUp),
            QualificationPoolKind::BestAtGroupPosition => {
                let count = self.count.ok_or_else(|| {
                    DbError::InvalidData("BestAtGroupPosition pool rule requires count".to_string())
                })?;
                let position_index = self.position_index.ok_or_else(|| {
                    DbError::InvalidData(
                        "BestAtGroupPosition pool rule requires position_index".to_string()
                    )
                })?;
                Ok(QualificationPoolRule::BestAtGroupPosition {
                    position_index: position_index as u32,
                    count: count as u32,
                })
            }
        }
    }
}
