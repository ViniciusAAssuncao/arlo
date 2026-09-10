use crate::error::{EngineError, EngineResult};
use arlo_domain::sport_constants::TOTAL_PLAYERS_PER_TEAM;
use arlo_domain::{Formation, FormationSlot, Player, Position, SlotRole};
use arlo_tactics::PlayerInstructions;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineupAssignment {
    formation_slot_index: usize,
    slot: FormationSlot,
    player: Arc<Player>,
    slot_role: SlotRole,
    player_instructions: PlayerInstructions,
}

impl LineupAssignment {
    pub fn new(
        formation_slot_index: usize,
        slot: FormationSlot,
        player: Arc<Player>,
        slot_role: SlotRole,
        player_instructions: PlayerInstructions,
    ) -> Self {
        Self {
            formation_slot_index,
            slot,
            player,
            slot_role,
            player_instructions,
        }
    }

    pub fn formation_slot_index(&self) -> usize {
        self.formation_slot_index
    }

    pub fn slot(&self) -> &FormationSlot {
        &self.slot
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn player_arc(&self) -> Arc<Player> {
        Arc::clone(&self.player)
    }

    pub fn slot_role(&self) -> SlotRole {
        self.slot_role
    }

    pub fn player_instructions(&self) -> PlayerInstructions {
        self.player_instructions
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lineup {
    formation: Formation,
    assignments: Vec<LineupAssignment>,
    players_cache: Vec<Arc<Player>>,
}

impl Lineup {
    pub fn new(formation: Formation, players: Vec<Player>) -> EngineResult<Self> {
        let expected_count = TOTAL_PLAYERS_PER_TEAM as usize;

        if players.len() != expected_count {
            return Err(EngineError::InvalidLineupSize {
                expected: expected_count,
                actual: players.len(),
            });
        }

        if formation.slots().len() != expected_count {
            return Err(EngineError::SlotCountMismatch {
                expected: expected_count,
                actual: formation.slots().len(),
            });
        }

        let mut seen = HashSet::with_capacity(expected_count);
        for p in &players {
            if !seen.insert(p.id()) {
                return Err(EngineError::DuplicatePlayer(p.id()));
            }
        }

        let assignments = formation
            .slots()
            .iter()
            .copied()
            .enumerate()
            .zip(players)
            .map(|((formation_slot_index, slot), player)| {
                LineupAssignment::new(
                    formation_slot_index,
                    slot,
                    Arc::new(player),
                    SlotRole::Standard,
                    PlayerInstructions::default(),
                )
            })
            .collect();

        Self::from_assignments(formation, assignments)
    }

    pub fn from_assignments(
        formation: Formation,
        assignments: Vec<LineupAssignment>,
    ) -> EngineResult<Self> {
        let expected_count = TOTAL_PLAYERS_PER_TEAM as usize;

        if assignments.len() != expected_count {
            return Err(EngineError::InvalidLineupSize {
                expected: expected_count,
                actual: assignments.len(),
            });
        }

        if formation.slots().len() != expected_count {
            return Err(EngineError::SlotCountMismatch {
                expected: expected_count,
                actual: formation.slots().len(),
            });
        }

        let mut seen = HashSet::with_capacity(expected_count);
        for a in &assignments {
            if !seen.insert(a.player().id()) {
                return Err(EngineError::DuplicatePlayer(a.player().id()));
            }
        }

        let players_cache = assignments
            .iter()
            .map(|a| Arc::clone(&a.player))
            .collect();

        Ok(Self {
            formation,
            assignments,
            players_cache,
        })
    }

    pub fn substitute(
        &self,
        outgoing_player_id: Uuid,
        incoming_player: Arc<Player>,
    ) -> EngineResult<Self> {
        let mut found = false;
        let mut new_assignments = Vec::with_capacity(self.assignments.len());
        for assignment in &self.assignments {
            if assignment.player().id() == outgoing_player_id {
                found = true;
                new_assignments.push(LineupAssignment::new(
                    assignment.formation_slot_index(),
                    *assignment.slot(),
                    Arc::clone(&incoming_player),
                    assignment.slot_role(),
                    assignment.player_instructions(),
                ));
            } else {
                new_assignments.push(assignment.clone());
            }
        }
        if !found {
            return Err(EngineError::PlayerNotFound(outgoing_player_id));
        }
        Self::from_assignments(self.formation.clone(), new_assignments)
    }

    pub fn builder(formation: Formation) -> LineupBuilder {
        LineupBuilder::new(formation)
    }

    pub fn formation(&self) -> &Formation {
        &self.formation
    }

    pub fn assignments(&self) -> &[LineupAssignment] {
        &self.assignments
    }

    pub fn players(&self) -> &[Arc<Player>] {
        &self.players_cache
    }

    pub fn role_index(&self) -> HashMap<Uuid, SlotRole> {
        self.assignments
            .iter()
            .map(|a| (a.player().id(), a.slot_role()))
            .collect()
    }

    pub fn instructions_index(&self) -> HashMap<Uuid, PlayerInstructions> {
        self.assignments
            .iter()
            .map(|a| (a.player().id(), a.player_instructions()))
            .collect()
    }

    pub fn offensive_position_index(&self) -> HashMap<Uuid, Position> {
        self.assignments
            .iter()
            .map(|a| (a.player().id(), a.slot().offensive_position()))
            .collect()
    }

    pub fn defensive_position_index(&self) -> HashMap<Uuid, Position> {
        self.assignments
            .iter()
            .map(|a| (a.player().id(), a.slot().defensive_position()))
            .collect()
    }

    pub fn position_index(&self) -> HashMap<Uuid, Position> {
        self.offensive_position_index()
    }

    pub fn get_slot_for_player(&self, player_id: &Uuid) -> Option<&FormationSlot> {
        self.assignments
            .iter()
            .find(|a| a.player().id() == *player_id)
            .map(|a| a.slot())
    }

    pub fn get_player_for_slot(&self, slot_index: usize) -> Option<&Player> {
        self.assignments.get(slot_index).map(|a| a.player())
    }

    pub fn player_at_slot_index(&self, slot_index: usize) -> Option<&Player> {
        self.assignments
            .iter()
            .find(|a| a.formation_slot_index() == slot_index)
            .map(|a| a.player())
    }

    pub fn slot_index_for_player(&self, player_id: &Uuid) -> Option<usize> {
        self.assignments
            .iter()
            .find(|a| a.player().id() == *player_id)
            .map(|a| a.formation_slot_index())
    }

    pub fn get_assignment(&self, player_id: &Uuid) -> Option<&LineupAssignment> {
        self.assignments
            .iter()
            .find(|a| a.player().id() == *player_id)
    }

    pub fn len(&self) -> usize {
        self.assignments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.assignments.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct LineupBuilder {
    formation: Formation,
    players: Vec<Player>,
}

impl LineupBuilder {
    pub fn new(formation: Formation) -> Self {
        Self {
            formation,
            players: Vec::with_capacity(TOTAL_PLAYERS_PER_TEAM as usize),
        }
    }

    pub fn with_player(mut self, player: Player) -> Self {
        self.players.push(player);
        self
    }

    pub fn with_players(mut self, players: impl IntoIterator<Item = Player>) -> Self {
        self.players.extend(players);
        self
    }

    pub fn build(self) -> EngineResult<Lineup> {
        Lineup::new(self.formation, self.players)
    }
}