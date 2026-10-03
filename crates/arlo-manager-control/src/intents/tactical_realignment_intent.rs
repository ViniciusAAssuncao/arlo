use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TacticalRealignmentIntent {
    first_player_id: Uuid,
    second_player_id: Uuid,
}

impl TacticalRealignmentIntent {
    pub fn new(first_player_id: Uuid, second_player_id: Uuid) -> Self {
        Self {
            first_player_id,
            second_player_id,
        }
    }

    pub fn first_player_id(&self) -> Uuid {
        self.first_player_id
    }

    pub fn second_player_id(&self) -> Uuid {
        self.second_player_id
    }
}
