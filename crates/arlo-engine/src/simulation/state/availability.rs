use arlo_events::AvailabilityStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AvailabilityState {
    Active,
    Suspended { remaining_seconds: f64 },
    Expelled,
    Injured,
}

impl AvailabilityState {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    pub fn is_suspended(&self) -> bool {
        matches!(self, Self::Suspended { .. })
    }

    pub fn is_expelled(&self) -> bool {
        matches!(self, Self::Expelled)
    }

    pub fn is_injured(&self) -> bool {
        matches!(self, Self::Injured)
    }

    pub fn remaining_suspension_seconds(&self) -> Option<f64> {
        match self {
            Self::Suspended { remaining_seconds } => Some(*remaining_seconds),
            _ => None,
        }
    }

    pub fn to_status(&self) -> (AvailabilityStatus, Option<f64>) {
        match self {
            Self::Active => (AvailabilityStatus::Active, None),
            Self::Suspended { remaining_seconds } => {
                (AvailabilityStatus::Suspended, Some(*remaining_seconds))
            }
            Self::Expelled => (AvailabilityStatus::Expelled, None),
            Self::Injured => (AvailabilityStatus::Injured, None),
        }
    }
}

impl Default for AvailabilityState {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerAvailabilityTracker {
    records: HashMap<Uuid, AvailabilityState>,
}

impl PlayerAvailabilityTracker {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }

    pub fn availability_for(&self, player_id: &Uuid) -> AvailabilityState {
        self.records.get(player_id).copied().unwrap_or(AvailabilityState::Active)
    }

    pub fn is_player_available(&self, player_id: &Uuid) -> bool {
        self.availability_for(player_id).is_active()
    }

    pub fn set_availability(&mut self, player_id: Uuid, state: AvailabilityState) {
        if state.is_active() {
            self.records.remove(&player_id);
        } else {
            self.records.insert(player_id, state);
        }
    }

    pub fn suspend_player(&mut self, player_id: Uuid, seconds: f64) {
        self.records.insert(
            player_id,
            AvailabilityState::Suspended {
                remaining_seconds: seconds.max(0.0),
            },
        );
    }

    pub fn expel_player(&mut self, player_id: Uuid) {
        self.records.insert(player_id, AvailabilityState::Expelled);
    }

    pub fn injure_player(&mut self, player_id: Uuid) {
        self.records.insert(player_id, AvailabilityState::Injured);
    }

    pub fn activate_player(&mut self, player_id: Uuid) {
        self.records.remove(&player_id);
    }

    pub fn restore_player_availability(&mut self, player_id: Uuid, state: AvailabilityState) {
        self.set_availability(player_id, state);
    }

    pub fn advance_time(&mut self, delta_seconds: f64) -> Vec<(Uuid, AvailabilityState, AvailabilityState)> {
        let mut transitions = Vec::new();
        let mut to_activate = Vec::new();

        for (&player_id, state) in &mut self.records {
            if let AvailabilityState::Suspended { remaining_seconds } = *state {
                let prev = *state;
                let next_seconds = remaining_seconds - delta_seconds;
                if next_seconds <= 0.0 {
                    to_activate.push((player_id, prev));
                } else {
                    *state = AvailabilityState::Suspended {
                        remaining_seconds: next_seconds,
                    };
                }
            }
        }

        for (player_id, prev) in to_activate {
            self.records.remove(&player_id);
            transitions.push((player_id, prev, AvailabilityState::Active));
        }

        transitions
    }

    pub fn records(&self) -> &HashMap<Uuid, AvailabilityState> {
        &self.records
    }

    pub fn clear(&mut self) {
        self.records.clear();
    }
}