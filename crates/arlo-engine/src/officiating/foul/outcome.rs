use crate::officiating::foul::origin::FoulOrigin;
use arlo_domain::PunishmentKind;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FoulResolution {
    pub offending_player_id: Uuid,
    pub offending_team_id: Uuid,
    pub opposing_player_id: Uuid,
    pub opposing_team_id: Uuid,
    pub origin: FoulOrigin,
    pub trigger_probability: f64,
    pub original_call_correct: bool,
    pub peace_referee_intervened: bool,
    pub fault_definition_id: Option<Uuid>,
    pub punishment_kind: Option<PunishmentKind>,
    pub punishment_magnitude: Option<i32>,
}

impl FoulResolution {
    pub fn new(
        offending_player_id: Uuid,
        offending_team_id: Uuid,
        opposing_player_id: Uuid,
        opposing_team_id: Uuid,
        origin: FoulOrigin,
        trigger_probability: f64,
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
            trigger_probability,
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
