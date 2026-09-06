use crate::resolution::outcome::DuelOutcome;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributedDuelOutcome {
    outcome: DuelOutcome,
    attacker_ids: Vec<Uuid>,
    defender_ids: Vec<Uuid>,
}

impl AttributedDuelOutcome {
    pub fn new(
        outcome: DuelOutcome,
        attacker_ids: Vec<Uuid>,
        defender_ids: Vec<Uuid>,
    ) -> Self {
        Self {
            outcome,
            attacker_ids,
            defender_ids,
        }
    }

    pub fn outcome(&self) -> &DuelOutcome {
        &self.outcome
    }

    pub fn attacker_ids(&self) -> &[Uuid] {
        &self.attacker_ids
    }

    pub fn defender_ids(&self) -> &[Uuid] {
        &self.defender_ids
    }
}