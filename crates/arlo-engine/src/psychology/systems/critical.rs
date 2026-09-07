use crate::psychology::state::ImpulseState;
use arlo_domain::sport_constants::impulse_floor_for_baseline;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const DEFAULT_CRITICAL_IMPULSE_DURATION_THRESHOLD_SECONDS: f64 = 300.0;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ImpulseCriticalReached {
    pub player_id: Uuid,
    pub impulse_value: u8,
    pub floor_value: f64,
    pub duration_below_floor_seconds: f64,
}

impl ImpulseCriticalReached {
    pub fn new(
        player_id: Uuid,
        impulse_value: u8,
        floor_value: f64,
        duration_below_floor_seconds: f64,
    ) -> Self {
        Self {
            player_id,
            impulse_value,
            floor_value,
            duration_below_floor_seconds,
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
        self.duration_below_floor_seconds
    }
}

pub fn is_impulse_critical(impulse_value: u8, baseline: f64) -> bool {
    let floor = impulse_floor_for_baseline(baseline);
    (impulse_value as f64) <= floor
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct ImpulseCriticalTracker {
    time_below_floor_seconds: f64,
    has_emitted: bool,
}

impl ImpulseCriticalTracker {
    pub fn new() -> Self {
        Self {
            time_below_floor_seconds: 0.0,
            has_emitted: false,
        }
    }

    pub fn update(
        &mut self,
        player_id: Uuid,
        impulse_state: &ImpulseState,
        baseline: f64,
        dt_seconds: f64,
        threshold_seconds: f64,
    ) -> Option<ImpulseCriticalReached> {
        let floor = impulse_floor_for_baseline(baseline);
        if (impulse_state.value() as f64) <= floor {
            self.time_below_floor_seconds += dt_seconds.max(0.0);
            if self.time_below_floor_seconds >= threshold_seconds && !self.has_emitted {
                self.has_emitted = true;
                return Some(ImpulseCriticalReached::new(
                    player_id,
                    impulse_state.value(),
                    floor,
                    self.time_below_floor_seconds,
                ));
            }
        } else {
            self.time_below_floor_seconds =
                (self.time_below_floor_seconds - dt_seconds.max(0.0) * 2.0).max(0.0);
            if self.time_below_floor_seconds == 0.0 {
                self.has_emitted = false;
            }
        }
        None
    }

    pub fn time_below_floor_seconds(&self) -> f64 {
        self.time_below_floor_seconds
    }

    pub fn has_emitted(&self) -> bool {
        self.has_emitted
    }

    pub fn reset(&mut self) {
        self.time_below_floor_seconds = 0.0;
        self.has_emitted = false;
    }
}