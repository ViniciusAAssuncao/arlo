use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TacticalSwitchIntent {
    pub profile_id: Uuid,
}

impl TacticalSwitchIntent {
    pub fn new(profile_id: Uuid) -> Self {
        Self { profile_id }
    }

    pub fn profile_id(&self) -> Uuid {
        self.profile_id
    }
}
