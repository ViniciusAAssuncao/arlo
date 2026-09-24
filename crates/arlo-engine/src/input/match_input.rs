use crate::error::{EngineError, EngineResult};
use crate::input::TeamInput;
use arlo_domain::{AttributeDefinition, AttributeTarget, MatchFormatRules, Pitch, Player, Referee};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MatchInput {
    match_id: Uuid,
    home: TeamInput,
    away: TeamInput,
    format: MatchFormatRules,
    pitch: Pitch,
    referees: Vec<Referee>,
    player_attribute_definitions: Vec<AttributeDefinition>,
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
        player_attribute_definitions: Vec<AttributeDefinition>,
        seed: u64,
    ) -> EngineResult<Self> {
        if home.team_id() == away.team_id() {
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
        let home_ids: HashSet<_> = home.roster().iter().map(Player::id).collect();
        if away
            .roster()
            .iter()
            .any(|player| home_ids.contains(&player.id()))
        {
            return Err(EngineError::InvalidInput(
                "player appears on both rosters".into(),
            ));
        }
        let mut attribute_ids = HashSet::new();
        let mut attribute_keys = HashSet::new();
        for definition in &player_attribute_definitions {
            if definition.applies_to() != AttributeTarget::Player
                || !attribute_ids.insert(definition.id())
                || !attribute_keys.insert(definition.key())
            {
                return Err(EngineError::InvalidInput(
                    "player attribute definitions must have unique IDs and keys".into(),
                ));
            }
        }
        Ok(Self {
            match_id,
            home,
            away,
            format,
            pitch,
            referees,
            player_attribute_definitions,
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
    pub fn player_attribute_definitions(&self) -> &[AttributeDefinition] {
        &self.player_attribute_definitions
    }
    pub fn seed(&self) -> u64 {
        self.seed
    }
}
