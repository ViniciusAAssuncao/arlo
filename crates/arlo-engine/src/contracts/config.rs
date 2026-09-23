use arlo_domain::{
    FaultCatalog, InjuryCatalog, Manager, MatchFormatRules, Pitch, Player, Referee, Team,
};
use arlo_tactics::{PlayCall, TacticalLineup, TeamTacticalProfile};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MatchScore {
    pub goal_points: u32,
    pub field_points: u32,
    pub field_goals_goalpost: u32,
    pub field_goals_fieldpost: u32,
    pub total_points: u32,
}

impl MatchScore {
    pub fn new(
        goal_points: u32,
        field_points: u32,
        field_goals_goalpost: u32,
        field_goals_fieldpost: u32,
    ) -> Self {
        let total_points =
            goal_points * 5 + field_points * 3 + field_goals_goalpost * 2 + field_goals_fieldpost;
        Self {
            goal_points,
            field_points,
            field_goals_goalpost,
            field_goals_fieldpost,
            total_points,
        }
    }

    pub fn goal_points(&self) -> u32 {
        self.goal_points
    }

    pub fn field_points(&self) -> u32 {
        self.field_points
    }

    pub fn field_goals(&self) -> u32 {
        self.field_goals_goalpost + self.field_goals_fieldpost
    }

    pub fn total_points(&self) -> u32 {
        self.total_points
    }

    pub fn add_goal_point(&mut self) {
        self.goal_points += 1;
        self.total_points += 5;
    }

    pub fn add_field_point(&mut self) {
        self.field_points += 1;
        self.total_points += 3;
    }

    pub fn add_field_goal_goalpost(&mut self) {
        self.field_goals_goalpost += 1;
        self.total_points += 2;
    }

    pub fn add_field_goal_fieldpost(&mut self) {
        self.field_goals_fieldpost += 1;
        self.total_points += 1;
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamSimulationConfig {
    pub team: Team,
    pub manager: Manager,
    pub players: Vec<Player>,
    pub lineup: TacticalLineup,
    pub tactical_profile: TeamTacticalProfile,
    pub available_profiles: Vec<TeamTacticalProfile>,
    pub playbook: Vec<PlayCall>,
}

impl TeamSimulationConfig {
    pub fn new(
        team: Team,
        manager: Manager,
        players: Vec<Player>,
        lineup: TacticalLineup,
        tactical_profile: TeamTacticalProfile,
        available_profiles: Vec<TeamTacticalProfile>,
        playbook: Vec<PlayCall>,
    ) -> Self {
        Self {
            team,
            manager,
            players,
            lineup,
            tactical_profile,
            available_profiles,
            playbook,
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team.id()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SimulationOptions {
    pub deterministic: bool,
    pub snapshot_interval_seconds: Option<f64>,
    pub max_events: Option<usize>,
    pub enable_injuries: bool,
    pub enable_officiating: bool,
    pub enable_impulse: bool,
    pub enable_fatigue: bool,
}

impl Default for SimulationOptions {
    fn default() -> Self {
        Self {
            deterministic: true,
            snapshot_interval_seconds: Some(60.0),
            max_events: None,
            enable_injuries: true,
            enable_officiating: true,
            enable_impulse: true,
            enable_fatigue: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationConfig {
    pub match_id: Uuid,
    pub home_team: TeamSimulationConfig,
    pub away_team: TeamSimulationConfig,
    pub head_referee: Referee,
    pub peace_referee: Referee,
    pub pitch: Pitch,
    pub format_rules: MatchFormatRules,
    pub fault_catalog: Arc<FaultCatalog>,
    pub injury_catalog: Arc<InjuryCatalog>,
    pub seed: u64,
    pub options: SimulationOptions,
}

impl SimulationConfig {
    pub fn new(
        match_id: Uuid,
        home_team: TeamSimulationConfig,
        away_team: TeamSimulationConfig,
        head_referee: Referee,
        peace_referee: Referee,
        pitch: Pitch,
        format_rules: MatchFormatRules,
        fault_catalog: Arc<FaultCatalog>,
        injury_catalog: Arc<InjuryCatalog>,
        seed: u64,
        options: SimulationOptions,
    ) -> Self {
        Self {
            match_id,
            home_team,
            away_team,
            head_referee,
            peace_referee,
            pitch,
            format_rules,
            fault_catalog,
            injury_catalog,
            seed,
            options,
        }
    }
}