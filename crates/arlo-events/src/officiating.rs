use crate::action::DuelKind;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoulRaised {
    offending_player_id: Uuid,
    offending_team_id: Uuid,
    opposing_player_id: Uuid,
    opposing_team_id: Uuid,
    duel_kind: DuelKind,
    original_call_correct: bool,
    peace_referee_intervened: bool,
}

impl FoulRaised {
    pub fn new(
        offending_player_id: Uuid,
        offending_team_id: Uuid,
        opposing_player_id: Uuid,
        opposing_team_id: Uuid,
        duel_kind: DuelKind,
        original_call_correct: bool,
        peace_referee_intervened: bool,
    ) -> Self {
        Self {
            offending_player_id,
            offending_team_id,
            opposing_player_id,
            opposing_team_id,
            duel_kind,
            original_call_correct,
            peace_referee_intervened,
        }
    }

    pub fn offending_player_id(&self) -> Uuid {
        self.offending_player_id
    }

    pub fn offending_team_id(&self) -> Uuid {
        self.offending_team_id
    }

    pub fn opposing_player_id(&self) -> Uuid {
        self.opposing_player_id
    }

    pub fn opposing_team_id(&self) -> Uuid {
        self.opposing_team_id
    }

    pub fn duel_kind(&self) -> DuelKind {
        self.duel_kind
    }

    pub fn original_call_correct(&self) -> bool {
        self.original_call_correct
    }

    pub fn peace_referee_intervened(&self) -> bool {
        self.peace_referee_intervened
    }

    pub fn final_call_correct(&self) -> bool {
        self.peace_referee_intervened || self.original_call_correct
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OfficiatingEvent {
    FoulRaised(FoulRaised),
}

impl From<FoulRaised> for OfficiatingEvent {
    fn from(ev: FoulRaised) -> Self {
        Self::FoulRaised(ev)
    }
}
