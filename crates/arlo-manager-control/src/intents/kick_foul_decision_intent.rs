use arlo_domain::KickFoulDecisionKind;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KickFoulDecisionIntent {
    decision: KickFoulDecisionKind,
    taker_id: Option<Uuid>,
}

impl KickFoulDecisionIntent {
    pub fn new(decision: KickFoulDecisionKind, taker_id: Option<Uuid>) -> Self {
        Self { decision, taker_id }
    }

    pub fn decision(&self) -> KickFoulDecisionKind {
        self.decision
    }

    pub fn taker_id(&self) -> Option<Uuid> {
        self.taker_id
    }
}
