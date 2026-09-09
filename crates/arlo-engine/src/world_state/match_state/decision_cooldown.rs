use crate::manager_ai::ManagerDecisionKind;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DecisionCooldownTracker {
    home: HashMap<ManagerDecisionKind, f64>,
    away: HashMap<ManagerDecisionKind, f64>,
}

impl DecisionCooldownTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn home(&self) -> &HashMap<ManagerDecisionKind, f64> {
        &self.home
    }

    pub fn away(&self) -> &HashMap<ManagerDecisionKind, f64> {
        &self.away
    }

    pub fn is_ready(
        &self,
        is_home: bool,
        kind: ManagerDecisionKind,
        current_time: f64,
        min_interval: f64,
    ) -> bool {
        let map = if is_home { &self.home } else { &self.away };
        match map.get(&kind) {
            Some(&last) => current_time - last >= min_interval,
            None => true,
        }
    }

    pub fn mark_triggered(
        &mut self,
        is_home: bool,
        kind: ManagerDecisionKind,
        current_time: f64,
    ) {
        let map = if is_home {
            &mut self.home
        } else {
            &mut self.away
        };
        map.insert(kind, current_time);
    }
}