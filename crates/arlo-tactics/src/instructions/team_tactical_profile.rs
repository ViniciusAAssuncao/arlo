use crate::instructions::TeamInstructions;
use crate::playcall::situational::{HasSituationalProfile, SituationalProfile};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamTacticalProfile {
    id: Uuid,
    team_id: Uuid,
    name: String,
    instructions: TeamInstructions,
    situational_profile: Option<SituationalProfile>,
    is_active: bool,
}

impl TeamTacticalProfile {
    pub fn new(
        id: Uuid,
        team_id: Uuid,
        name: impl Into<String>,
        instructions: TeamInstructions,
        situational_profile: Option<SituationalProfile>,
        is_active: bool,
    ) -> Self {
        Self {
            id,
            team_id,
            name: name.into(),
            instructions,
            situational_profile,
            is_active,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn instructions(&self) -> &TeamInstructions {
        &self.instructions
    }

    pub fn situational_profile(&self) -> Option<&SituationalProfile> {
        self.situational_profile.as_ref()
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }
}

impl HasSituationalProfile for TeamTacticalProfile {
    fn situational_profile(&self) -> Option<&SituationalProfile> {
        self.situational_profile.as_ref()
    }
}
