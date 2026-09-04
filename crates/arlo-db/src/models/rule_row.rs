use crate::error::{DbError, DbResult};
use arlo_domain::{Rule, RuleCategory};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct RuleRow {
    pub id: String,
    pub competition_id: String,
    pub category: String,
    pub rule_key: String,
    pub value: String,
}

impl RuleRow {
    pub fn to_domain(&self) -> DbResult<Rule> {
        let id = Uuid::parse_str(&self.id)?;
        let competition_id = Uuid::parse_str(&self.competition_id)?;
        let category = match self.category.as_str() {
            "Calendar" => RuleCategory::Calendar,
            "Teams" => RuleCategory::Teams,
            "Scoring" => RuleCategory::Scoring,
            "Phases" => RuleCategory::Phases,
            "TieBreaker" => RuleCategory::TieBreaker,
            "PromotionRelegation" => RuleCategory::PromotionRelegation,
            _ => {
                return Err(DbError::InvalidEnum(format!(
                    "Invalid rule category: {}",
                    self.category
                )))
            }
        };
        Rule::new(id, competition_id, category, &self.rule_key, &self.value).map_err(Into::into)
    }
}