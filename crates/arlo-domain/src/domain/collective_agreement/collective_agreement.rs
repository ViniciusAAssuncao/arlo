use crate::domain::collective_agreement::collective_agreement_rule::CollectiveAgreementRule;
use crate::domain::validation::validate_not_empty;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectiveAgreement {
    id: Uuid,
    name: String,
    rule: CollectiveAgreementRule,
}

impl CollectiveAgreement {
    pub fn new(
        id: Uuid,
        name: impl Into<String>,
        rule: CollectiveAgreementRule,
    ) -> DomainResult<Self> {
        let name = name.into();
        validate_not_empty(&name, "name")?;

        Ok(Self { id, name, rule })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rule(&self) -> &CollectiveAgreementRule {
        &self.rule
    }
}
