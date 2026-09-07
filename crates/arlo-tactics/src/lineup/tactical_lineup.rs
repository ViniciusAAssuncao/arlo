use crate::lineup::builder::TacticalLineupBuilder;
use crate::lineup::slot_assignment::SlotAssignment;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TacticalLineup {
    id: Uuid,
    team_id: Uuid,
    formation_id: Uuid,
    name: String,
    assignments: Vec<SlotAssignment>,
}

impl TacticalLineup {
    pub fn new(
        id: Uuid,
        team_id: Uuid,
        formation_id: Uuid,
        name: impl Into<String>,
        assignments: Vec<SlotAssignment>,
    ) -> Self {
        Self {
            id,
            team_id,
            formation_id,
            name: name.into(),
            assignments,
        }
    }

    pub fn builder(id: Uuid, team_id: Uuid, name: impl Into<String>) -> TacticalLineupBuilder {
        TacticalLineupBuilder::new(id, team_id, name)
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn formation_id(&self) -> Uuid {
        self.formation_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn assignments(&self) -> &[SlotAssignment] {
        &self.assignments
    }

    pub fn assignment_for_slot(&self, slot_index: usize) -> Option<&SlotAssignment> {
        self.assignments
            .iter()
            .find(|a| a.formation_slot_index() == slot_index)
    }

    pub fn assignment_for_player(&self, player_id: Uuid) -> Option<&SlotAssignment> {
        self.assignments.iter().find(|a| a.player_id() == player_id)
    }
}