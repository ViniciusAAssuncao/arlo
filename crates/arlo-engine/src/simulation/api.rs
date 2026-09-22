use crate::error::{ EngineError, EngineResult };
use crate::simulation::{
    SimulationPeriodicSnapshot,
    SimulationPlayerSnapshot,
    SimulationTeamSnapshot,
};
use arlo_domain::{
    FaultCatalog,
    InjuryCatalog,
    Manager,
    MatchFormatRules,
    Pitch,
    Player,
    Referee,
    Team,
};
use arlo_events::{ MatchClockInstant, MatchEventEnvelope };
use arlo_manager_control::{ ManagerDecisionInbox, RequiredManagerDecision };
use arlo_tactics::{ PlayCall, TacticalLineup, TeamTacticalProfile };
use serde::{ Deserialize, Serialize };
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
        field_goals_fieldpost: u32
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
        playbook: Vec<PlayCall>
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
        options: SimulationOptions
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SimulationStatus {
    Ready,
    InProgress,
    PausedForDecision(RequiredManagerDecision),
    Completed,
    Aborted(String),
}

impl SimulationStatus {
    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Ready)
    }

    pub fn is_in_progress(&self) -> bool {
        matches!(self, Self::InProgress)
    }

    pub fn is_paused(&self) -> bool {
        matches!(self, Self::PausedForDecision(_))
    }

    pub fn is_completed(&self) -> bool {
        matches!(self, Self::Completed)
    }

    pub fn is_aborted(&self) -> bool {
        matches!(self, Self::Aborted(_))
    }

    pub fn pending_decision(&self) -> Option<&RequiredManagerDecision> {
        match self {
            Self::PausedForDecision(d) => Some(d),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationStepResult {
    pub status: SimulationStatus,
    pub events: Vec<MatchEventEnvelope>,
    pub clock: MatchClockInstant,
    pub home_score: MatchScore,
    pub away_score: MatchScore,
    pub pending_decision: Option<RequiredManagerDecision>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationOutput {
    pub match_id: Uuid,
    pub status: SimulationStatus,
    pub final_clock: MatchClockInstant,
    pub home_score: MatchScore,
    pub away_score: MatchScore,
    pub winner_team_id: Option<Uuid>,
    pub total_events: usize,
    pub total_periods_played: u32,
    pub events: Vec<MatchEventEnvelope>,
    pub player_snapshots: Vec<SimulationPlayerSnapshot>,
    pub team_snapshots: Vec<SimulationTeamSnapshot>,
    pub periodic_snapshots: Vec<SimulationPeriodicSnapshot>,
    pub pending_decision: Option<RequiredManagerDecision>,
}

impl SimulationOutput {
    pub fn is_completed(&self) -> bool {
        self.status.is_completed()
    }

    pub fn is_draw(&self) -> bool {
        self.winner_team_id.is_none() && self.is_completed()
    }
}

pub trait MatchSimulator {
    fn config(&self) -> &SimulationConfig;
    fn status(&self) -> SimulationStatus;
    fn current_clock(&self) -> MatchClockInstant;
    fn current_score(&self) -> (MatchScore, MatchScore);
    fn pending_decision(&self) -> Option<&RequiredManagerDecision>;
    fn step(&mut self, inbox: &ManagerDecisionInbox) -> EngineResult<SimulationStepResult>;
    fn run_to_completion(&mut self, inbox: &ManagerDecisionInbox) -> EngineResult<SimulationOutput>;
    fn capture_snapshot(&self) -> SimulationPeriodicSnapshot;
}

#[derive(Debug, Clone)]
pub struct EventDrivenMatchSimulator {
    config: SimulationConfig,
    status: SimulationStatus,
    clock: MatchClockInstant,
    home_score: MatchScore,
    away_score: MatchScore,
    events: Vec<MatchEventEnvelope>,
    sequence_counter: u64,
}

impl EventDrivenMatchSimulator {
    pub fn new(config: SimulationConfig) -> EngineResult<Self> {
        let clock = MatchClockInstant::zero();
        Ok(Self {
            config,
            status: SimulationStatus::Ready,
            clock,
            home_score: MatchScore::default(),
            away_score: MatchScore::default(),
            events: Vec::new(),
            sequence_counter: 0,
        })
    }
}

impl MatchSimulator for EventDrivenMatchSimulator {
    fn config(&self) -> &SimulationConfig {
        &self.config
    }

    fn status(&self) -> SimulationStatus {
        self.status.clone()
    }

    fn current_clock(&self) -> MatchClockInstant {
        self.clock
    }

    fn current_score(&self) -> (MatchScore, MatchScore) {
        (self.home_score, self.away_score)
    }

    fn pending_decision(&self) -> Option<&RequiredManagerDecision> {
        self.status.pending_decision()
    }

    fn step(&mut self, _inbox: &ManagerDecisionInbox) -> EngineResult<SimulationStepResult> {
        if self.status.is_completed() {
            return Err(EngineError::SimulationCompleted);
        }

        self.status = SimulationStatus::Completed;

        Ok(SimulationStepResult {
            status: self.status.clone(),
            events: Vec::new(),
            clock: self.clock,
            home_score: self.home_score,
            away_score: self.away_score,
            pending_decision: None,
        })
    }

    fn run_to_completion(
        &mut self,
        inbox: &ManagerDecisionInbox
    ) -> EngineResult<SimulationOutput> {
        while !self.status.is_completed() && !self.status.is_aborted() && !self.status.is_paused() {
            self.step(inbox)?;
        }

        let winner_team_id = if self.home_score.total_points > self.away_score.total_points {
            Some(self.config.home_team.team_id())
        } else if self.away_score.total_points > self.home_score.total_points {
            Some(self.config.away_team.team_id())
        } else {
            None
        };

        Ok(SimulationOutput {
            match_id: self.config.match_id,
            status: self.status.clone(),
            final_clock: self.clock,
            home_score: self.home_score,
            away_score: self.away_score,
            winner_team_id,
            total_events: self.events.len(),
            total_periods_played: self.clock.period(),
            events: self.events.clone(),
            player_snapshots: Vec::new(),
            team_snapshots: Vec::new(),
            periodic_snapshots: Vec::new(),
            pending_decision: self.status.pending_decision().cloned(),
        })
    }

    fn capture_snapshot(&self) -> SimulationPeriodicSnapshot {
        SimulationPeriodicSnapshot {
            sequence_number: self.sequence_counter,
            clock: self.clock,
            player_snapshots: Vec::new(),
            team_snapshots: Vec::new(),
        }
    }
}
