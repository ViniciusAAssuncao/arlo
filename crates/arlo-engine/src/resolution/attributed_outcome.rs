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

    pub fn with_active_duelists(
        outcome: DuelOutcome,
        primary_attacker_id: Uuid,
        active_helpers: Vec<Uuid>,
        primary_defender_id: Uuid,
        active_defenders: Vec<Uuid>,
    ) -> Self {
        let mut attackers = vec![primary_attacker_id];
        for id in active_helpers {
            if !attackers.contains(&id) {
                attackers.push(id);
            }
        }
        let mut defenders = vec![primary_defender_id];
        for id in active_defenders {
            if !defenders.contains(&id) {
                defenders.push(id);
            }
        }
        Self::new(outcome, attackers, defenders)
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

    pub fn primary_attacker_id(&self) -> Option<Uuid> {
        self.attacker_ids.first().copied()
    }

    pub fn primary_defender_id(&self) -> Option<Uuid> {
        self.defender_ids.first().copied()
    }

    pub fn is_active_attacker(&self, player_id: &Uuid) -> bool {
        self.attacker_ids.contains(player_id)
    }

    pub fn is_active_defender(&self, player_id: &Uuid) -> bool {
        self.defender_ids.contains(player_id)
    }

    pub fn is_active_participant(&self, player_id: &Uuid) -> bool {
        self.is_active_attacker(player_id) || self.is_active_defender(player_id)
    }
}