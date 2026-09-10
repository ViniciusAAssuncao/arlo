use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TacticalProfileActivated {
    team_id: Uuid,
    profile_id: Uuid,
    profile_name: String,
}

impl TacticalProfileActivated {
    pub fn new(team_id: Uuid, profile_id: Uuid, profile_name: impl Into<String>) -> Self {
        Self {
            team_id,
            profile_id,
            profile_name: profile_name.into(),
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn profile_id(&self) -> Uuid {
        self.profile_id
    }

    pub fn profile_name(&self) -> &str {
        &self.profile_name
    }
}
