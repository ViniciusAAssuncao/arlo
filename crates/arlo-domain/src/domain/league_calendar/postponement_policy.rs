use crate::domain::league_calendar::postponement_strategy_kind::PostponementStrategyKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PostponementPolicy {
    strategy: PostponementStrategyKind,
}

impl PostponementPolicy {
    pub fn new(strategy: PostponementStrategyKind) -> Self {
        Self { strategy }
    }

    pub fn strategy(&self) -> PostponementStrategyKind {
        self.strategy
    }
}
