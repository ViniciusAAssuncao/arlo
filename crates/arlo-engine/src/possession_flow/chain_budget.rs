use crate::possession_flow::chain_state::ChainState;
use uuid::Uuid;

pub const DEFAULT_CHAIN_SAFETY_BUDGET: usize = 8;

pub fn initialize_chain_budget(initial_carrier_id: Uuid) -> ChainState {
    ChainState::new(initial_carrier_id, DEFAULT_CHAIN_SAFETY_BUDGET)
}

pub fn is_budget_exhausted(state: &ChainState) -> bool {
    !state.has_budget()
}
