use arlo_domain::PunishmentKind;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PunishmentApplied {
    offending_player_id: Uuid,
    offending_team_id: Uuid,
    fault_definition_id: Option<Uuid>,
    kind: PunishmentKind,
    magnitude: Option<i32>,
}

impl PunishmentApplied {
    pub fn new(
        offending_player_id: Uuid,
        offending_team_id: Uuid,
        fault_definition_id: Option<Uuid>,
        kind: PunishmentKind,
        magnitude: Option<i32>,
    ) -> Self {
        Self { offending_player_id, offending_team_id, fault_definition_id, kind, magnitude }
    }

    pub fn offending_player_id(&self) -> Uuid { self.offending_player_id }
    pub fn offending_team_id(&self) -> Uuid { self.offending_team_id }
    pub fn fault_definition_id(&self) -> Option<Uuid> { self.fault_definition_id }
    pub fn kind(&self) -> PunishmentKind { self.kind }
    pub fn magnitude(&self) -> Option<i32> { self.magnitude }
}
