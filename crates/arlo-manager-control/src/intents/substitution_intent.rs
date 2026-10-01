use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SubstitutionIntent {
    pub outgoing_player_id: Uuid,
    pub incoming_player_id: Uuid,
}

impl SubstitutionIntent {
    pub fn new(outgoing_player_id: Uuid, incoming_player_id: Uuid) -> Self {
        Self {
            outgoing_player_id,
            incoming_player_id,
        }
    }

    pub fn outgoing_player_id(&self) -> Uuid {
        self.outgoing_player_id
    }

    pub fn incoming_player_id(&self) -> Uuid {
        self.incoming_player_id
    }
}
