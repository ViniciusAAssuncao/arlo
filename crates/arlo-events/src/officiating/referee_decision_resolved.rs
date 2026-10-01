use crate::officiating::FoulOrigin;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefereeDecisionResolved {
    offending_player_id: Uuid,
    offending_team_id: Uuid,
    opposing_player_id: Uuid,
    opposing_team_id: Uuid,
    origin: FoulOrigin,
    fault_definition_id: Option<Uuid>,
    factual_foul: bool,
    original_call: bool,
    peace_referee_intervened: bool,
}

impl RefereeDecisionResolved {
    pub fn new(
        offending_player_id: Uuid,
        offending_team_id: Uuid,
        opposing_player_id: Uuid,
        opposing_team_id: Uuid,
        origin: FoulOrigin,
        fault_definition_id: Option<Uuid>,
        factual_foul: bool,
        original_call: bool,
        peace_referee_intervened: bool,
    ) -> Self {
        Self {
            offending_player_id,
            offending_team_id,
            opposing_player_id,
            opposing_team_id,
            origin,
            fault_definition_id,
            factual_foul,
            original_call,
            peace_referee_intervened,
        }
    }

    pub fn offending_player_id(&self) -> Uuid { self.offending_player_id }
    pub fn offending_team_id(&self) -> Uuid { self.offending_team_id }
    pub fn opposing_player_id(&self) -> Uuid { self.opposing_player_id }
    pub fn opposing_team_id(&self) -> Uuid { self.opposing_team_id }
    pub fn origin(&self) -> FoulOrigin { self.origin }
    pub fn fault_definition_id(&self) -> Option<Uuid> { self.fault_definition_id }
    pub fn factual_foul(&self) -> bool { self.factual_foul }
    pub fn original_call(&self) -> bool { self.original_call }
    pub fn peace_referee_intervened(&self) -> bool { self.peace_referee_intervened }
    pub fn final_call(&self) -> bool {
        if self.peace_referee_intervened { self.factual_foul } else { self.original_call }
    }
    pub fn original_call_correct(&self) -> bool { self.original_call == self.factual_foul }
    pub fn final_call_correct(&self) -> bool { self.final_call() == self.factual_foul }
}
