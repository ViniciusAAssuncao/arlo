use crate::domain::invariant_violation::InvariantViolation;
use crate::domain::league_calendar::qualification_pool_rule::QualificationPoolRule;
use crate::domain::league_calendar::qualification_pool_rule_validation::validate_qualification_pool_rule;
use crate::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageEntryRule {
    pools: Vec<QualificationPoolRule>,
}

impl StageEntryRule {
    pub fn new(pools: Vec<QualificationPoolRule>) -> DomainResult<Self> {
        if pools.is_empty() {
            return Err(DomainError::InvalidInvariant {
                field: "pools".to_string(),
                violation: InvariantViolation::Empty,
            });
        }

        for pool in &pools {
            validate_qualification_pool_rule(pool)?;
        }

        Ok(Self { pools })
    }

    pub fn all_teams() -> Self {
        Self {
            pools: vec![QualificationPoolRule::AllTeams],
        }
    }

    pub fn top_n(count: u32) -> DomainResult<Self> {
        Self::new(vec![QualificationPoolRule::TopN { count }])
    }

    pub fn bottom_n(count: u32) -> DomainResult<Self> {
        Self::new(vec![QualificationPoolRule::BottomN { count }])
    }

    pub fn pools(&self) -> &[QualificationPoolRule] {
        &self.pools
    }

    pub fn requires_groups(&self) -> bool {
        self.pools.iter().any(|p| p.requires_groups())
    }
}
