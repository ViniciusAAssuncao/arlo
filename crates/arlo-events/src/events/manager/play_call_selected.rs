use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlayCallCategory {
    OpenPlay,
    BonusPhaseConversion,
}

impl PlayCallCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OpenPlay => "OpenPlay",
            Self::BonusPhaseConversion => "BonusPhaseConversion",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayCallSelected {
    team_id: Uuid,
    play_call_id: Uuid,
    play_call_name: String,
    category: PlayCallCategory,
}

impl PlayCallSelected {
    pub fn new(
        team_id: Uuid,
        play_call_id: Uuid,
        play_call_name: impl Into<String>,
        category: PlayCallCategory,
    ) -> Self {
        Self {
            team_id,
            play_call_id,
            play_call_name: play_call_name.into(),
            category,
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn play_call_id(&self) -> Uuid {
        self.play_call_id
    }

    pub fn play_call_name(&self) -> &str {
        &self.play_call_name
    }

    pub fn category(&self) -> PlayCallCategory {
        self.category
    }
}
