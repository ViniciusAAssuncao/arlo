use crate::error::{EngineError, EngineResult};
use arlo_domain::{Formation, Manager, MatchFormatRules, Pitch, Player, Position, Referee};
use arlo_tactics::{validate_tactical_lineup, TacticalLineup, TeamTacticalProfile};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TeamInput {
    team_id: Uuid,
    formation: Formation,
    lineup: TacticalLineup,
    roster: Vec<Player>,
    manager: Manager,
    tactics: TeamTacticalProfile,
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
            let slot = formation
                .slots()
                .get(assignment.formation_slot_index())
                .ok_or_else(|| {
                    EngineError::InvalidInput("lineup references an unknown slot".into())
                })?;
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
            if assignment.position() != slot.offensive_position() {
                return Err(EngineError::InvalidInput(
                    "lineup position differs from formation slot".into(),
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
        if artrines != 1 || passers != 1 || goalguards != 1 {
            return Err(EngineError::InvalidInput(
                "lineup requires one Artrine, Passer and Goalguard".into(),
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
        })
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
}

#[derive(Debug, Clone)]
pub struct MatchInput {
    match_id: Uuid,
    home: TeamInput,
    away: TeamInput,
    format: MatchFormatRules,
    pitch: Pitch,
    referees: Vec<Referee>,
    seed: u64,
}

impl MatchInput {
    pub fn new(
        match_id: Uuid,
        home: TeamInput,
        away: TeamInput,
        format: MatchFormatRules,
        pitch: Pitch,
        referees: Vec<Referee>,
        seed: u64,
    ) -> EngineResult<Self> {
        if home.team_id == away.team_id {
            return Err(EngineError::InvalidInput(
                "home and away teams must differ".into(),
            ));
        }
        if format.regulation_periods() != 4
            || format.regulation_period_duration_minutes() != 30
            || format.allows_overtime()
        {
            return Err(EngineError::InvalidInput(
                "engine currently supports four 30-minute quarters without tie overtime".into(),
            ));
        }
        if referees.is_empty() {
            return Err(EngineError::InvalidInput(
                "match requires at least one referee".into(),
            ));
        }
        let mut referee_ids = HashSet::new();
        if referees
            .iter()
            .any(|referee| !referee_ids.insert(referee.id()))
        {
            return Err(EngineError::InvalidInput(
                "referees must be distinct".into(),
            ));
        }
        let home_ids: HashSet<_> = home.roster.iter().map(Player::id).collect();
        if away
            .roster
            .iter()
            .any(|player| home_ids.contains(&player.id()))
        {
            return Err(EngineError::InvalidInput(
                "player appears on both rosters".into(),
            ));
        }
        Ok(Self {
            match_id,
            home,
            away,
            format,
            pitch,
            referees,
            seed,
        })
    }

    pub fn match_id(&self) -> Uuid {
        self.match_id
    }
    pub fn home(&self) -> &TeamInput {
        &self.home
    }
    pub fn away(&self) -> &TeamInput {
        &self.away
    }
    pub fn format(&self) -> MatchFormatRules {
        self.format
    }
    pub fn pitch(&self) -> Pitch {
        self.pitch
    }
    pub fn referees(&self) -> &[Referee] {
        &self.referees
    }
    pub fn seed(&self) -> u64 {
        self.seed
    }
}
