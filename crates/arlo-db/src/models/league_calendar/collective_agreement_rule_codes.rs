use crate::error::{DbError, DbResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectiveAgreementRuleKind {
    AnnualBlackoutWindow,
}

pub fn parse_collective_agreement_rule_kind(code: &str) -> DbResult<CollectiveAgreementRuleKind> {
    match code {
        "AnnualBlackoutWindow" | "annual_blackout_window" => {
            Ok(CollectiveAgreementRuleKind::AnnualBlackoutWindow)
        }
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid collective agreement rule kind: {code}"
        ))),
    }
}

pub fn collective_agreement_rule_kind_to_code(kind: CollectiveAgreementRuleKind) -> &'static str {
    match kind {
        CollectiveAgreementRuleKind::AnnualBlackoutWindow => "AnnualBlackoutWindow",
    }
}
