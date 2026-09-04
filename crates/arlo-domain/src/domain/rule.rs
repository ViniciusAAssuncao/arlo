use crate::domain::validation::validate_not_empty;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RuleCategory {
    Calendar,
    Teams,
    Scoring,
    Phases,
    TieBreaker,
    PromotionRelegation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rule {
    id: Uuid,
    competition_id: Uuid,
    category: RuleCategory,
    rule_key: String,
    value: String,
}

impl Rule {
    pub fn new(
        id: Uuid,
        competition_id: Uuid,
        category: RuleCategory,
        rule_key: impl Into<String>,
        value: impl Into<String>,
    ) -> DomainResult<Self> {
        let rule_key = rule_key.into();
        let value = value.into();
        validate_not_empty(&rule_key, "rule_key")?;
        validate_not_empty(&value, "value")?;

        Ok(Self {
            id,
            competition_id,
            category,
            rule_key,
            value,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn competition_id(&self) -> Uuid {
        self.competition_id
    }

    pub fn category(&self) -> RuleCategory {
        self.category
    }

    pub fn rule_key(&self) -> &str {
        &self.rule_key
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
