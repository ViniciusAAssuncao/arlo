use crate::world_state::match_state::state::MatchState;
use arlo_manager_control::RequiredManagerDecision;

impl MatchState {
    pub fn peek_pending_manager_decisions(&self) -> Vec<RequiredManagerDecision> {
        crate::world_state::step::readiness::peek_pending_manager_decisions(self)
    }
}