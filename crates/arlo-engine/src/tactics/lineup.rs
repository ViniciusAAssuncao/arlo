use crate::error::{EngineError, EngineResult};
use arlo_domain::sport_constants::TOTAL_PLAYERS_PER_TEAM;
use arlo_domain::{Formation, FormationSlot, Player, Position};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineupAssignment {
    slot: FormationSlot,
    player: Player,
}

impl LineupAssignment {
    pub fn new(slot: FormationSlot, player: Player) -> Self {
        Self { slot, player }
    }

    pub fn slot(&self) -> &FormationSlot {
        &self.slot
    }

    pub fn player(&self) -> &Player {
        &self.player
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lineup {
    formation: Formation,
    assignments: Vec<LineupAssignment>,
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
            .zip(players.into_iter())
            .map(|(slot, player)| LineupAssignment::new(slot, player))
            .collect();

        Ok(Self {
            formation,
            assignments,
        })
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

        Ok(Self {
            formation,
            assignments,
        })
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

    pub fn players(&self) -> Vec<&Player> {
        self.assignments.iter().map(|a| a.player()).collect()
    }

    pub fn position_index(&self) -> HashMap<Uuid, Position> {
        self.assignments
            .iter()
            .map(|a| (a.player().id(), a.slot().position()))
            .collect()
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
