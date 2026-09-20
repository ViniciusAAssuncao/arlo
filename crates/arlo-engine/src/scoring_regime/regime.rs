use crate::scoring_regime::policy::ScoringRegimePolicy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct ScoringRegime {
    pub policy: ScoringRegimePolicy,
}

impl ScoringRegime {
    pub fn new(policy: ScoringRegimePolicy) -> Self {
        Self { policy }
    }

    pub fn policy(&self) -> &ScoringRegimePolicy {
        &self.policy
    }
}
