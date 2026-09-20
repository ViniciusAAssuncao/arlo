use crate::psychology::state::ImpulseState;
use arlo_domain::sport_constants::impulse_floor_for_baseline;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ImpulseCriticalReached {
    player_id: Uuid,
    impulse_value: u8,
    floor_value: f64,
}

impl ImpulseCriticalReached {
    pub fn new(player_id: Uuid, impulse_value: u8, floor_value: f64) -> Self {
        Self {
            player_id,
            impulse_value,
            floor_value,
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn impulse_value(&self) -> u8 {
        self.impulse_value
    }

    pub fn floor_value(&self) -> f64 {
        self.floor_value
    }

    pub fn duration_below_floor_seconds(&self) -> f64 {
        0.0
    }
}

pub fn is_impulse_critical(impulse_value: u8, baseline: f64) -> bool {
    let floor = impulse_floor_for_baseline(baseline);
    (impulse_value as f64) <= floor
}

pub fn is_moral_collapse(state: &ImpulseState) -> bool {
    is_impulse_critical(state.value(), state.baseline())
}
