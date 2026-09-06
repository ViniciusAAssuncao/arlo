use arlo_domain::sport_constants::{
    MAX_CALL_TO_ACTIONS_PER_SERIES, MINIMUM_ADVANCE_MIRINS_PER_SERIES,
};
use arlo_math::units::Position;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeriesState {
    down: u8,
    advanced_mirins: f64,
    scrimmage_point: Position,
    pub is_bonus_phase: bool,
}

impl SeriesState {
    pub fn new(down: u8, advanced_mirins: f64, scrimmage_point: Position) -> Self {
        Self {
            down,
            advanced_mirins,
            scrimmage_point,
            is_bonus_phase: false,
        }
    }

    pub fn with_bonus_phase(
        down: u8,
        advanced_mirins: f64,
        scrimmage_point: Position,
        is_bonus_phase: bool,
    ) -> Self {
        Self {
            down,
            advanced_mirins,
            scrimmage_point,
            is_bonus_phase,
        }
    }

    pub fn initial(scrimmage_point: Position) -> Self {
        Self {
            down: 1,
            advanced_mirins: 0.0,
            scrimmage_point,
            is_bonus_phase: false,
        }
    }

    pub fn down(&self) -> u8 {
        self.down
    }

    pub fn advanced_mirins(&self) -> f64 {
        self.advanced_mirins
    }

    pub fn scrimmage_point(&self) -> Position {
        self.scrimmage_point
    }

    pub fn line_of_scrimmage(&self) -> Position {
        self.scrimmage_point
    }

    pub fn is_bonus_phase(&self) -> bool {
        self.is_bonus_phase
    }

    pub fn set_bonus_phase(&mut self, is_bonus_phase: bool) {
        self.is_bonus_phase = is_bonus_phase;
    }

    pub fn set_scrimmage_point(&mut self, new_scrimmage: Position) {
        self.scrimmage_point = new_scrimmage;
    }

    pub fn record_advance(&mut self, mirins: f64) {
        self.advanced_mirins += mirins;
    }

    pub fn advance_down(&mut self) -> bool {
        if self.down < MAX_CALL_TO_ACTIONS_PER_SERIES as u8 {
            self.down += 1;
            true
        } else {
            false
        }
    }

    pub fn has_achieved_target(&self) -> bool {
        self.advanced_mirins >= MINIMUM_ADVANCE_MIRINS_PER_SERIES as f64
    }

    pub fn is_last_down(&self) -> bool {
        self.down >= MAX_CALL_TO_ACTIONS_PER_SERIES as u8
    }

    pub fn should_turnover_on_downs(&self) -> bool {
        self.is_last_down() && !self.has_achieved_target()
    }

    pub fn reset(&mut self, new_scrimmage: Position) {
        self.down = 1;
        self.advanced_mirins = 0.0;
        self.scrimmage_point = new_scrimmage;
        self.is_bonus_phase = false;
    }

    pub fn remaining_mirins_to_target(&self) -> f64 {
        (MINIMUM_ADVANCE_MIRINS_PER_SERIES as f64 - self.advanced_mirins).max(0.0)
    }

    pub fn remaining_downs(&self) -> u8 {
        (MAX_CALL_TO_ACTIONS_PER_SERIES as u8).saturating_sub(self.down)
    }
}
