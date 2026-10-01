use crate::error::{TacticsError, TacticsResult};
use crate::instructions::player::PlayerInstructions;
use crate::lineup::slot_assignment::SlotAssignment;
use crate::lineup::tactical_lineup::TacticalLineup;
use crate::lineup::validation::validate_tactical_lineup;
use arlo_domain::{Formation, Player, SlotRole};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TacticalLineupBuilder {
    id: Uuid,
    team_id: Uuid,
    formation: Option<Formation>,
    name: String,
    assignments: HashMap<usize, Uuid>,
    roles: HashMap<Uuid, SlotRole>,
    instructions: HashMap<Uuid, PlayerInstructions>,
}

impl TacticalLineupBuilder {
    pub fn new(id: Uuid, team_id: Uuid, name: impl Into<String>) -> Self {
        Self {
            id,
            team_id,
            formation: None,
            name: name.into(),
            assignments: HashMap::new(),
            roles: HashMap::new(),
            instructions: HashMap::new(),
        }
    }

    pub fn with_formation(mut self, formation: &Formation) -> Self {
        self.formation = Some(formation.clone());
        self
    }

    pub fn assign(mut self, slot_index: usize, player_id: Uuid) -> Self {
        self.assignments.insert(slot_index, player_id);
        self
    }

    pub fn designate_role(mut self, player_id: Uuid, role: SlotRole) -> Self {
        self.roles.insert(player_id, role);
        self
    }

    pub fn instruct(mut self, player_id: Uuid, instructions: PlayerInstructions) -> Self {
        self.instructions.insert(player_id, instructions);
        self
    }

    pub fn build(self, roster: &[Player]) -> TacticsResult<TacticalLineup> {
        let formation = self.formation.ok_or_else(|| {
            TacticsError::InvalidLineup("Formation is required to build a lineup".to_string())
        })?;

        let mut assignments = Vec::with_capacity(formation.slots().len());
        for (idx, slot) in formation.slots().iter().enumerate() {
            if let Some(&player_id) = self.assignments.get(&idx) {
                let role = self
                    .roles
                    .get(&player_id)
                    .copied()
                    .unwrap_or(SlotRole::Standard);
                let instructions = self
                    .instructions
                    .get(&player_id)
                    .copied()
                    .unwrap_or_default();
                assignments.push(SlotAssignment::new(
                    idx,
                    slot.position(),
                    player_id,
                    role,
                    instructions,
                ));
            }
        }

        let lineup = TacticalLineup::new(
            self.id,
            self.team_id,
            formation.id(),
            self.name,
            assignments,
        );

        validate_tactical_lineup(&lineup, &formation, roster)?;

        Ok(lineup)
    }
}
