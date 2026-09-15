use crate::error::{DbError, DbResult};
use crate::models::league_calendar::entry_rule_pool_codes::{
    parse_qualification_pool_kind, QualificationPoolKind,
};
use arlo_domain::QualificationPoolRule;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct EntryRulePoolRow {
    pub id: String,
    pub league_calendar_stage_definition_id: String,
    pub pool_order_index: i32,
    pub pool_kind: String,
    pub count: Option<i32>,
    pub position_index: Option<i32>,
    pub range_start_position: Option<i32>,
    pub range_end_position: Option<i32>,
    pub external_competition_id: Option<String>,
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
                        "BestAtGroupPosition pool rule requires position_index".to_string(),
                    )
                })?;
                Ok(QualificationPoolRule::BestAtGroupPosition {
                    position_index: position_index as u32,
                    count: count as u32,
                })
            }
            QualificationPoolKind::PositionRange => {
                let start = self.range_start_position.ok_or_else(|| {
                    DbError::InvalidData(
                        "PositionRange pool rule requires range_start_position".to_string(),
                    )
                })?;
                let end = self.range_end_position.ok_or_else(|| {
                    DbError::InvalidData(
                        "PositionRange pool rule requires range_end_position".to_string(),
                    )
                })?;
                Ok(QualificationPoolRule::PositionRange {
                    start_position: start as u32,
                    end_position: end as u32,
                })
            }
            QualificationPoolKind::ExternalCompetitionWinner => {
                let comp_id_str = self.external_competition_id.as_deref().ok_or_else(|| {
                    DbError::InvalidData(
                        "ExternalCompetitionWinner pool rule requires external_competition_id".to_string(),
                    )
                })?;
                let competition_id = Uuid::parse_str(comp_id_str)?;
                Ok(QualificationPoolRule::ExternalCompetitionWinner { competition_id })
            }
        }
    }
}
