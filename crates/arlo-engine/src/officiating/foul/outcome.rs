use crate::resolution::DuelKind;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FoulResolution {
    pub offending_player_id: Uuid,
    pub offending_team_id: Uuid,
    pub opposing_player_id: Uuid,
    pub opposing_team_id: Uuid,
    pub engine_duel_kind: DuelKind,
    pub trigger_probability: f64,
    pub original_call_correct: bool,
    pub peace_referee_intervened: bool,
}

impl FoulResolution {
    pub fn new(
        offending_player_id: Uuid,
        offending_team_id: Uuid,
        opposing_player_id: Uuid,
        opposing_team_id: Uuid,
        engine_duel_kind: DuelKind,
        trigger_probability: f64,
        original_call_correct: bool,
        peace_referee_intervened: bool,
    ) -> Self {
        Self {
            offending_player_id,
            offending_team_id,
            opposing_player_id,
            opposing_team_id,
            engine_duel_kind,
            trigger_probability,
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

    pub fn engine_duel_kind(&self) -> DuelKind {
        self.engine_duel_kind
    }

    pub fn trigger_probability(&self) -> f64 {
        self.trigger_probability
    }

    pub fn original_call_correct(&self) -> bool {
        self.original_call_correct
    }

    pub fn peace_referee_intervened(&self) -> bool {
        self.peace_referee_intervened
    }

    pub fn final_call_correct(&self) -> bool {
        if self.peace_referee_intervened {
            true
        } else {
            self.original_call_correct
        }
    }
}
