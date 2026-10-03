use arlo_domain::{Position, SlotRole};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TacticalAssignment {
    pub player_id: Uuid,
    pub formation_slot_index: usize,
    pub offensive_position: Position,
    pub defensive_position: Position,
    pub slot_role: SlotRole,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TacticalRealignmentMade {
    team_id: Uuid,
    assignments: [TacticalAssignment; 2],
}

impl TacticalRealignmentMade {
    pub fn new(team_id: Uuid, assignments: [TacticalAssignment; 2]) -> Self {
        Self {
            team_id,
            assignments,
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn assignments(&self) -> &[TacticalAssignment; 2] {
        &self.assignments
    }
}
