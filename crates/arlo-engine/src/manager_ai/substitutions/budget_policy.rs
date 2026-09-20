use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubstitutionBudgetPolicy {
    max_subs_per_stoppage: usize,
    max_subs_per_period: usize,
}

impl SubstitutionBudgetPolicy {
    pub fn new(max_subs_per_stoppage: usize, max_subs_per_period: usize) -> Self {
        Self {
            max_subs_per_stoppage,
            max_subs_per_period,
        }
    }

    pub fn max_subs_per_stoppage(&self) -> usize {
        self.max_subs_per_stoppage
    }

    pub fn max_subs_per_period(&self) -> usize {
        self.max_subs_per_period
    }
}

impl Default for SubstitutionBudgetPolicy {
    fn default() -> Self {
        Self {
            max_subs_per_stoppage: 2,
            max_subs_per_period: 4,
        }
    }
}
