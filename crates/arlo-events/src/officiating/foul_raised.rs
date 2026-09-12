use crate::officiating::foul_origin::FoulOrigin;
use arlo_domain::PunishmentKind;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FoulRaised {
    offending_player_id: Uuid,
    offending_team_id: Uuid,
    opposing_player_id: Uuid,
    opposing_team_id: Uuid,
    origin: FoulOrigin,
    original_call_correct: bool,
    peace_referee_intervened: bool,
    fault_definition_id: Option<Uuid>,
    punishment_kind: Option<PunishmentKind>,
    punishment_magnitude: Option<i32>,
}

impl FoulRaised {
    pub fn new(
        offending_player_id: Uuid,
        offending_team_id: Uuid,
        opposing_player_id: Uuid,
        opposing_team_id: Uuid,
        origin: FoulOrigin,
        original_call_correct: bool,
        peace_referee_intervened: bool,
        fault_definition_id: Option<Uuid>,
        punishment_kind: Option<PunishmentKind>,
        punishment_magnitude: Option<i32>,
    ) -> Self {
        Self {
            offending_player_id,
            offending_team_id,
            opposing_player_id,
            opposing_team_id,
            origin,
            original_call_correct,
            peace_referee_intervened,
            fault_definition_id,
            punishment_kind,
            punishment_magnitude,
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

    pub fn origin(&self) -> FoulOrigin {
        self.origin
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

    pub fn fault_definition_id(&self) -> Option<Uuid> {
        self.fault_definition_id
    }

    pub fn punishment_kind(&self) -> Option<PunishmentKind> {
        self.punishment_kind
    }

    pub fn punishment_magnitude(&self) -> Option<i32> {
        self.punishment_magnitude
    }
}
