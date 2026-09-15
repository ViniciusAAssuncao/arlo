use crate::error::DbResult;
use crate::models::league_calendar::tie_break_criterion_code::parse_tie_break_criterion;
use arlo_domain::TieBreakCriterion;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct TieBreakCriterionRow {
    pub id: String,
    pub league_calendar_config_id: String,
    pub order_index: i32,
    pub criterion_kind: String,
}

impl TieBreakCriterionRow {
    pub fn to_domain(&self) -> DbResult<TieBreakCriterion> {
        parse_tie_break_criterion(&self.criterion_kind)
    }
}
