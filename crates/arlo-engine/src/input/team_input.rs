use crate::error::{EngineError, EngineResult};
use arlo_domain::{Formation, Manager, Player, Position};
use arlo_tactics::{validate_tactical_lineup, TacticalLineup, TeamTacticalProfile};
use std::collections::HashSet;
use uuid::Uuid;

mod plans;

#[derive(Debug, Clone)]
pub struct TeamInput {
    team_id: Uuid,
    formation: Formation,
    lineup: TacticalLineup,
    roster: Vec<Player>,
    manager: Manager,
    tactics: TeamTacticalProfile,
    alternative_tactics: Vec<TeamTacticalProfile>,
    prepared_plans: Vec<arlo_tactics::PreparedTacticalPlan>,
}

impl TeamInput {
    pub fn new(
        team_id: Uuid,
        formation: Formation,
        lineup: TacticalLineup,
        roster: Vec<Player>,
        manager: Manager,
        tactics: TeamTacticalProfile,
    ) -> EngineResult<Self> {
        if lineup.team_id() != team_id || tactics.team_id() != team_id {
            return Err(EngineError::InvalidInput(
                "lineup or tactics belongs to another team".into(),
            ));
        }
        if lineup.formation_id() != formation.id() {
            return Err(EngineError::InvalidInput(
                "lineup references another formation".into(),
            ));
        }
        if manager.team_id().is_some_and(|id| id != team_id) {
            return Err(EngineError::InvalidInput(
                "manager belongs to another team".into(),
            ));
        }
        if lineup.assignments().len() != formation.slots().len() {
            return Err(EngineError::InvalidInput(
                "lineup must fill all formation slots".into(),
            ));
        }
        if formation.slots().len() != 14 {
            return Err(EngineError::InvalidInput(
                "formation must contain 14 slots".into(),
            ));
        }

        let mut roster_ids = HashSet::new();
        for player in &roster {
            if !roster_ids.insert(player.id()) {
                return Err(EngineError::InvalidInput(
                    "roster contains a duplicate player".into(),
                ));
            }
            if player.team_id().is_some_and(|id| id != team_id) {
                return Err(EngineError::InvalidInput(
                    "roster contains a player from another team".into(),
                ));
            }
        }

        let mut assigned_players = HashSet::new();
        let mut assigned_slots = HashSet::new();
        for assignment in lineup.assignments() {
            if formation
                .slots()
                .get(assignment.formation_slot_index())
                .is_none()
            {
                return Err(EngineError::InvalidInput(
                    "lineup references an unknown slot".into(),
                ));
            }
            if !assigned_players.insert(assignment.player_id())
                || !assigned_slots.insert(assignment.formation_slot_index())
            {
                return Err(EngineError::InvalidInput(
                    "lineup repeats a player or slot".into(),
                ));
            }
            if !roster_ids.contains(&assignment.player_id()) {
                return Err(EngineError::InvalidInput(
                    "lineup player is missing from roster".into(),
                ));
            }
        }

        let artrines = lineup
            .assignments()
            .iter()
            .filter(|a| a.position() == Position::Artrine)
            .count();
        let passers = lineup
            .assignments()
            .iter()
            .filter(|a| a.position() == Position::Passer)
            .count();
        let goalguards = lineup
            .assignments()
            .iter()
            .filter(|a| a.position() == Position::Goalguard)
            .count();
        if artrines == 0 || passers == 0 || goalguards == 0 {
            return Err(EngineError::InvalidInput(
                "lineup requires at least one Artrine, Passer and Goalguard".into(),
            ));
        }
        validate_tactical_lineup(&lineup, &formation, &roster)
            .map_err(|error| EngineError::InvalidInput(error.to_string()))?;

        Ok(Self {
            team_id,
            formation,
            lineup,
            roster,
            manager,
            tactics,
            alternative_tactics: Vec::new(),
            prepared_plans: Vec::new(),
        })
    }

    pub fn with_alternative_tactics(
        mut self,
        alternatives: Vec<TeamTacticalProfile>,
    ) -> EngineResult<Self> {
        let mut ids = HashSet::from([self.tactics.id()]);
        for profile in &alternatives {
            if profile.team_id() != self.team_id || !ids.insert(profile.id()) {
                return Err(EngineError::InvalidInput(
                    "invalid alternative tactical profile".into(),
                ));
            }
        }
        self.alternative_tactics = alternatives;
        Ok(self)
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }
    pub fn formation(&self) -> &Formation {
        &self.formation
    }
    pub fn lineup(&self) -> &TacticalLineup {
        &self.lineup
    }
    pub fn roster(&self) -> &[Player] {
        &self.roster
    }
    pub fn manager(&self) -> &Manager {
        &self.manager
    }
    pub fn tactics(&self) -> &TeamTacticalProfile {
        &self.tactics
    }
    pub fn tactical_profiles(&self) -> impl Iterator<Item = &TeamTacticalProfile> {
        std::iter::once(&self.tactics).chain(self.alternative_tactics.iter())
    }
}
