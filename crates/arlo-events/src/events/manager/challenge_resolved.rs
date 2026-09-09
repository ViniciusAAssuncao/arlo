use serde::{ Deserialize, Serialize };
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReviewableCallKind {
    TurnoverClassification,
    OutOfBoundsClassification,
    DriveValidity,
}

impl ReviewableCallKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TurnoverClassification => "TurnoverClassification",
            Self::OutOfBoundsClassification => "OutOfBoundsClassification",
            Self::DriveValidity => "DriveValidity",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChallengeResolved {
    team_id: Uuid,
    call_kind: ReviewableCallKind,
    success: bool,
    remaining_challenges_after: u32,
}

impl ChallengeResolved {
    pub fn new(
        team_id: Uuid,
        call_kind: ReviewableCallKind,
        success: bool,
        remaining_challenges_after: u32
    ) -> Self {
        Self {
            team_id,
            call_kind,
            success,
            remaining_challenges_after,
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn call_kind(&self) -> ReviewableCallKind {
        self.call_kind
    }

    pub fn success(&self) -> bool {
        self.success
    }

    pub fn remaining_challenges_after(&self) -> u32 {
        self.remaining_challenges_after
    }
}
