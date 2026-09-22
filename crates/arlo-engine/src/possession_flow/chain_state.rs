use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChainState {
    pub total_mirins_advanced: f64,
    pub touch_index: usize,
    pub current_carrier_id: Uuid,
    pub remaining_budget: usize,
}

impl ChainState {
    pub fn new(initial_carrier_id: Uuid, budget: usize) -> Self {
        Self {
            total_mirins_advanced: 0.0,
            touch_index: 0,
            current_carrier_id: initial_carrier_id,
            remaining_budget: budget,
        }
    }

    pub fn total_mirins_advanced(&self) -> f64 {
        self.total_mirins_advanced
    }

    pub fn touch_index(&self) -> usize {
        self.touch_index
    }

    pub fn current_carrier_id(&self) -> Uuid {
        self.current_carrier_id
    }

    pub fn remaining_budget(&self) -> usize {
        self.remaining_budget
    }

    pub fn has_budget(&self) -> bool {
        self.remaining_budget > 0
    }

    pub fn record_touch(&mut self, next_carrier_id: Uuid, mirins_advanced: f64) {
        self.total_mirins_advanced += mirins_advanced;
        self.touch_index += 1;
        self.current_carrier_id = next_carrier_id;
        self.remaining_budget = self.remaining_budget.saturating_sub(1);
    }
}
