use crate::error::{EngineError, EngineResult};
use crate::input::TeamInput;
use arlo_domain::{AttributeDefinition, AttributeKey, AttributeTarget, FaultCatalog, InjuryCatalog, MatchFormatRules, Pitch, Player, Referee};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MatchInput {
    match_id: Uuid,
    home: TeamInput,
    away: TeamInput,
    format: MatchFormatRules,
    pitch: Pitch,
    referees: Vec<Referee>,
    referee_attribute_keys: Arc<HashMap<Uuid, AttributeKey>>,
    fault_catalog: Arc<FaultCatalog>,
    injury_catalog: Arc<InjuryCatalog>,
    player_attribute_definitions: Vec<AttributeDefinition>,
    manager_attribute_keys: Arc<HashMap<Uuid, AttributeKey>>,
    player_start_energy: Arc<HashMap<Uuid, f64>>,
    player_start_morale: Arc<HashMap<Uuid, f64>>,
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
        referee_attribute_keys: Arc<HashMap<Uuid, AttributeKey>>,
        fault_catalog: Arc<FaultCatalog>,
        injury_catalog: Arc<InjuryCatalog>,
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
        if referees.len() != 2 {
            return Err(EngineError::InvalidInput(
                "match requires a head referee and a peace referee".into(),
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
            referee_attribute_keys,
            fault_catalog,
            injury_catalog,
            player_attribute_definitions,
            manager_attribute_keys: Arc::new(HashMap::new()),
            player_start_energy: Arc::new(HashMap::new()),
            player_start_morale: Arc::new(HashMap::new()),
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
    pub fn referee_attribute_keys(&self) -> &HashMap<Uuid, AttributeKey> {
        &self.referee_attribute_keys
    }
    pub fn fault_catalog(&self) -> &FaultCatalog {
        &self.fault_catalog
    }
    pub fn injury_catalog(&self) -> &InjuryCatalog {
        &self.injury_catalog
    }
    pub fn player_attribute_definitions(&self) -> &[AttributeDefinition] {
        &self.player_attribute_definitions
    }
    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn with_manager_decision_context(
        mut self,
        manager_attribute_keys: Arc<HashMap<Uuid, AttributeKey>>,
        player_start_energy: HashMap<Uuid, f64>,
    ) -> EngineResult<Self> {
        if player_start_energy.values().any(|energy| !energy.is_finite() || !(0.0..=1.0).contains(energy)) {
            return Err(EngineError::InvalidInput("player start energy must be between zero and one".into()));
        }
        self.manager_attribute_keys = manager_attribute_keys;
        self.player_start_energy = Arc::new(player_start_energy);
        Ok(self)
    }

    pub fn manager_attribute_keys(&self) -> &HashMap<Uuid, AttributeKey> {
        &self.manager_attribute_keys
    }

    pub fn player_start_energy(&self, player_id: Uuid) -> f64 {
        self.player_start_energy.get(&player_id).copied().unwrap_or(1.0)
    }

    pub fn with_player_start_morale(mut self, values: HashMap<Uuid, f64>) -> EngineResult<Self> {
        if values.values().any(|value| !value.is_finite() || !(0.0..=120.0).contains(value)) {
            return Err(EngineError::InvalidInput("player start morale must be between zero and 120".into()));
        }
        self.player_start_morale = Arc::new(values);
        Ok(self)
    }

    pub fn player_start_morale(&self, player_id: Uuid) -> f64 {
        self.player_start_morale.get(&player_id).copied().unwrap_or(100.0)
    }
}
