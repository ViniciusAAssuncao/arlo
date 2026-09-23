use crate::compatibility::seed_view::MatchSeed;
use arlo_domain::{
    AttributeKey, FaultCatalog, Formation, InjuryCatalog, Manager, MatchFormatRules, Pitch,
    Player, Referee,
};
use arlo_tactics::{PlayCall, TacticalLineup, TeamTacticalProfile};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct TeamSetupParams {
    pub team_id: Uuid,
    pub lineup: TacticalLineup,
    pub formation: Formation,
    pub players: Vec<Player>,
    pub profile: TeamTacticalProfile,
    pub manager: Manager,
    pub available_profiles: Vec<TeamTacticalProfile>,
    pub playbook: Vec<PlayCall>,
}

impl TeamSetupParams {
    pub fn new(
        team_id: Uuid,
        lineup: TacticalLineup,
        formation: Formation,
        players: Vec<Player>,
        profile: TeamTacticalProfile,
        manager: Manager,
        available_profiles: Vec<TeamTacticalProfile>,
        playbook: Vec<PlayCall>,
    ) -> Self {
        Self {
            team_id,
            lineup,
            formation,
            players,
            profile,
            manager,
            available_profiles,
            playbook,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatchSetupParams {
    pub home_team: TeamSetupParams,
    pub away_team: TeamSetupParams,
    pub head_referee: Referee,
    pub peace_referee: Referee,
    pub pitch: Pitch,
    pub attribute_keys: HashMap<Uuid, AttributeKey>,
    pub format_rules: MatchFormatRules,
    pub fault_catalog: Arc<FaultCatalog>,
    pub injury_catalog: Arc<InjuryCatalog>,
    pub seed: MatchSeed,
}

impl MatchSetupParams {
    pub fn new(
        home_team: TeamSetupParams,
        away_team: TeamSetupParams,
        head_referee: Referee,
        peace_referee: Referee,
        pitch: Pitch,
        attribute_keys: HashMap<Uuid, AttributeKey>,
        format_rules: MatchFormatRules,
        fault_catalog: Arc<FaultCatalog>,
        injury_catalog: Arc<InjuryCatalog>,
        seed: MatchSeed,
    ) -> Self {
        Self {
            home_team,
            away_team,
            head_referee,
            peace_referee,
            pitch,
            attribute_keys,
            format_rules,
            fault_catalog,
            injury_catalog,
            seed,
        }
    }
}