use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum BonusPhaseState {
    #[default]
    Inactive,
    Active,
}

impl BonusPhaseState {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}
