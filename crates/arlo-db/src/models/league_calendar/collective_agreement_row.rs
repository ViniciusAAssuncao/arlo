use crate::error::DbResult;
use crate::models::league_calendar::collective_agreement_rule_codes::{
    parse_collective_agreement_rule_kind, CollectiveAgreementRuleKind,
};
use arlo_domain::{AnnualBlackoutWindow, CollectiveAgreement, CollectiveAgreementRule};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct CollectiveAgreementRow {
    pub id: String,
    pub name: String,
    pub rule_kind: String,
    pub start_month_order_index: i32,
    pub start_day_of_month: i32,
    pub end_month_order_index: i32,
    pub end_day_of_month: i32,
    pub end_year_offset: i32,
    pub created_at_unix_seconds: i64,
}

impl CollectiveAgreementRow {
    pub fn to_domain(&self) -> DbResult<CollectiveAgreement> {
        let id = Uuid::parse_str(&self.id)?;
        let kind = parse_collective_agreement_rule_kind(&self.rule_kind)?;
        let rule = match kind {
            CollectiveAgreementRuleKind::AnnualBlackoutWindow => {
                let window = AnnualBlackoutWindow::new(
                    self.start_month_order_index as u32,
                    self.start_day_of_month as u32,
                    self.end_month_order_index as u32,
                    self.end_day_of_month as u32,
                    self.end_year_offset as u32,
                )?;
                CollectiveAgreementRule::AnnualBlackoutWindow(window)
            }
        };
        CollectiveAgreement::new(id, &self.name, rule).map_err(Into::into)
    }
}