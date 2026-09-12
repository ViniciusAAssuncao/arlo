use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum AvailabilityState {
    #[default]
    Active,
    Suspended {
        remaining_seconds: f64,
    },
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
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerAvailabilityTracker {
    home_availability: HashMap<Uuid, AvailabilityState>,
    away_availability: HashMap<Uuid, AvailabilityState>,
}

impl PlayerAvailabilityTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn home_availability(&self) -> &HashMap<Uuid, AvailabilityState> {
        &self.home_availability
    }

    pub fn away_availability(&self) -> &HashMap<Uuid, AvailabilityState> {
        &self.away_availability
    }

    pub fn availability_for(&self, id: &Uuid) -> AvailabilityState {
        self.home_availability
            .get(id)
            .or_else(|| self.away_availability.get(id))
            .copied()
            .unwrap_or(AvailabilityState::Active)
    }

    pub fn suspend_player(&mut self, player_id: Uuid, is_home: bool, remaining_seconds: f64) {
        let map = if is_home {
            &mut self.home_availability
        } else {
            &mut self.away_availability
        };
        map.insert(
            player_id,
            AvailabilityState::Suspended {
                remaining_seconds: remaining_seconds.max(0.0),
            },
        );
    }

    pub fn expel_player(&mut self, player_id: Uuid, is_home: bool) {
        let map = if is_home {
            &mut self.home_availability
        } else {
            &mut self.away_availability
        };
        map.insert(player_id, AvailabilityState::Expelled);
    }

    pub fn injure_player(&mut self, player_id: Uuid, is_home: bool) {
        let map = if is_home {
            &mut self.home_availability
        } else {
            &mut self.away_availability
        };
        map.insert(player_id, AvailabilityState::Injured);
    }

    pub fn restore_player(&mut self, player_id: Uuid, is_home: bool, state: AvailabilityState) {
        let map = if is_home {
            &mut self.home_availability
        } else {
            &mut self.away_availability
        };
        map.insert(player_id, state);
    }

    pub fn substitute_player(&mut self, _outgoing: Uuid, incoming: Uuid, is_home: bool) {
        let map = if is_home {
            &mut self.home_availability
        } else {
            &mut self.away_availability
        };
        if !map.contains_key(&incoming) {
            map.insert(incoming, AvailabilityState::Active);
        }
    }

    pub fn tick(
        &mut self,
        dt_seconds: f64,
    ) -> Vec<(Uuid, bool, AvailabilityState, AvailabilityState)> {
        if dt_seconds <= 0.0 {
            return Vec::new();
        }

        let mut transitions = Vec::new();

        for (&id, state) in self.home_availability.iter_mut() {
            let current = *state;
            if let AvailabilityState::Suspended { remaining_seconds } = current {
                let next_remaining = remaining_seconds - dt_seconds;
                if next_remaining <= 0.0 {
                    *state = AvailabilityState::Active;
                    transitions.push((id, true, current, AvailabilityState::Active));
                } else {
                    *state = AvailabilityState::Suspended {
                        remaining_seconds: next_remaining,
                    };
                }
            }
        }

        for (&id, state) in self.away_availability.iter_mut() {
            let current = *state;
            if let AvailabilityState::Suspended { remaining_seconds } = current {
                let next_remaining = remaining_seconds - dt_seconds;
                if next_remaining <= 0.0 {
                    *state = AvailabilityState::Active;
                    transitions.push((id, false, current, AvailabilityState::Active));
                } else {
                    *state = AvailabilityState::Suspended {
                        remaining_seconds: next_remaining,
                    };
                }
            }
        }

        transitions
    }
}