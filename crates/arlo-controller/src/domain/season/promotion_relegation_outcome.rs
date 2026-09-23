use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PromotionRelegationOutcome {
    promoted_team_ids: Vec<Uuid>,
    relegated_team_ids: Vec<Uuid>,
}

impl PromotionRelegationOutcome {
    pub fn new(promoted_team_ids: Vec<Uuid>, relegated_team_ids: Vec<Uuid>) -> Self {
        Self {
            promoted_team_ids,
            relegated_team_ids,
        }
    }

    pub fn empty() -> Self {
        Self {
            promoted_team_ids: Vec::new(),
            relegated_team_ids: Vec::new(),
        }
    }

    pub fn promoted_team_ids(&self) -> &[Uuid] {
        &self.promoted_team_ids
    }

    pub fn relegated_team_ids(&self) -> &[Uuid] {
        &self.relegated_team_ids
    }
}
