use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlayCallIntent {
    pub play_call_id: Uuid,
}

impl PlayCallIntent {
    pub fn new(play_call_id: Uuid) -> Self {
        Self { play_call_id }
    }

    pub fn play_call_id(&self) -> Uuid {
        self.play_call_id
    }
}
