use crate::mock::build_mock_match_state;
use crate::validation::{run_full_validation_and_simulation, ValidationResults};
use crate::MatchSession;
use arlo_engine::match_decision::DetailedPlayOutcome;
use arlo_engine::EngineResult;
use arlo_stats::player::{PlayerDrivesAggregator, PlayerDuelAggregator, PlayerTouchesAggregator};
use arlo_stats::AggregatorRegistry;

pub struct GameSimulationSession {
    session: MatchSession,
}

impl GameSimulationSession {
    pub fn new_mock(seed: u64) -> Self {
        let (state, _, _, _) = build_mock_match_state(seed);
        let mut registry = AggregatorRegistry::new();
        registry.register_aggregator(PlayerDuelAggregator::new());
        registry.register_aggregator(PlayerDrivesAggregator::new());
        registry.register_aggregator(PlayerTouchesAggregator::new());
        let session = MatchSession::new(state, registry);
        Self { session }
    }

    pub fn session(&self) -> &MatchSession {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut MatchSession {
        &mut self.session
    }

    pub fn run_match(&mut self) -> EngineResult<Vec<DetailedPlayOutcome>> {
        self.session.step_until_finished()
    }

    pub fn run_validation_suite() -> ValidationResults {
        run_full_validation_and_simulation()
    }
}