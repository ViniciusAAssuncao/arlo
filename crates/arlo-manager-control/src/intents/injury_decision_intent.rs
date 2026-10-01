use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct InjuryDecisionIntent {
    injured_player_id: Uuid,
    withdraw: bool,
    replacement_player_id: Option<Uuid>,
}

impl InjuryDecisionIntent {
    pub fn keep(injured_player_id: Uuid) -> Self {
        Self { injured_player_id, withdraw: false, replacement_player_id: None }
    }

    pub fn withdraw(injured_player_id: Uuid, replacement_player_id: Uuid) -> Self {
        Self { injured_player_id, withdraw: true, replacement_player_id: Some(replacement_player_id) }
    }

    pub fn withdraw_without_replacement(injured_player_id: Uuid) -> Self {
        Self { injured_player_id, withdraw: true, replacement_player_id: None }
    }

    pub fn injured_player_id(&self) -> Uuid {
        self.injured_player_id
    }

    pub fn replacement_player_id(&self) -> Option<Uuid> {
        self.replacement_player_id
    }

    pub fn withdraws_player(&self) -> bool {
        self.withdraw
    }
}
