use crate::contracts::config::{MatchScore, SimulationConfig};
use crate::contracts::output::SimulationOutput;
use crate::contracts::periodic_snapshot::SimulationPeriodicSnapshot;
use crate::contracts::status::SimulationStatus;
use crate::contracts::step_result::SimulationStepResult;
use crate::error::{EngineError, EngineResult};
use arlo_events::MatchClockInstant;
use arlo_manager_control::{ManagerDecisionInbox, RequiredManagerDecision};

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

        Err(EngineError::NotImplemented(
            "EventDrivenMatchSimulator step execution is not yet implemented",
        ))
    }

    fn run_to_completion(
        &mut self,
        _inbox: &ManagerDecisionInbox,
    ) -> EngineResult<SimulationOutput> {
        if self.status.is_completed() {
            return Err(EngineError::SimulationCompleted);
        }

        Err(EngineError::NotImplemented(
            "EventDrivenMatchSimulator full run is not yet implemented",
        ))
    }

    fn capture_snapshot(&self) -> SimulationPeriodicSnapshot {
        SimulationPeriodicSnapshot {
            sequence_number: 0,
            clock: self.clock,
            player_snapshots: Vec::new(),
            team_snapshots: Vec::new(),
        }
    }
}