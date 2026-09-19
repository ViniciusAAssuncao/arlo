use crate::possession::bonus_phase::{BonusPhasePolicy, BonusPhaseState};
use arlo_domain::sport_constants::{
    MAX_CALL_TO_ACTIONS_PER_SERIES, MINIMUM_ADVANCE_MIRINS_PER_SERIES,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeriesState {
    down: u8,
    advanced_mirins: f64,
    scrimmage_x_mirim: f64,
    pub bonus_phase: BonusPhaseState,
}

impl SeriesState {
    pub fn new(down: u8, advanced_mirins: f64, scrimmage_x_mirim: f64) -> Self {
        Self {
            down,
            advanced_mirins,
            scrimmage_x_mirim,
            bonus_phase: BonusPhaseState::Inactive,
        }
    }

    pub fn with_bonus_phase(
        down: u8,
        advanced_mirins: f64,
        scrimmage_x_mirim: f64,
        bonus_phase: BonusPhaseState,
    ) -> Self {
        Self {
            down,
            advanced_mirins,
            scrimmage_x_mirim,
            bonus_phase,
        }
    }

    pub fn initial(scrimmage_x_mirim: f64) -> Self {
        Self {
            down: 1,
            advanced_mirins: 0.0,
            scrimmage_x_mirim,
            bonus_phase: BonusPhaseState::Inactive,
        }
    }

    pub fn down(&self) -> u8 {
        self.down
    }

    pub fn advanced_mirins(&self) -> f64 {
        self.advanced_mirins
    }

    pub fn scrimmage_x_mirim(&self) -> f64 {
        self.scrimmage_x_mirim
    }

    pub fn is_bonus_phase(&self) -> bool {
        self.bonus_phase.is_active()
    }

    pub fn set_bonus_phase(&mut self, bonus_phase: bool) {
        self.bonus_phase = if bonus_phase {
            BonusPhaseState::Active
        } else {
            BonusPhaseState::Inactive
        };
    }

    pub fn set_scrimmage_x_mirim(&mut self, new_scrimmage_x_mirim: f64) {
        self.scrimmage_x_mirim = new_scrimmage_x_mirim;
    }

    pub fn record_advance(&mut self, mirins: f64) {
        self.advanced_mirins += mirins;
    }

    pub fn max_downs(&self) -> u8 {
        if self.is_bonus_phase() {
            BonusPhasePolicy::default().max_plays
        } else {
            MAX_CALL_TO_ACTIONS_PER_SERIES as u8
        }
    }

    pub fn advance_down(&mut self) -> bool {
        if self.down < self.max_downs() {
            self.down += 1;
            true
        } else {
            false
        }
    }

    pub fn has_achieved_target(&self) -> bool {
        self.advanced_mirins >= MINIMUM_ADVANCE_MIRINS_PER_SERIES
    }

    pub fn is_last_down(&self) -> bool {
        self.down >= self.max_downs()
    }

    pub fn should_turnover_on_downs(&self) -> bool {
        if self.is_bonus_phase() {
            self.is_last_down()
        } else {
            self.is_last_down() && !self.has_achieved_target()
        }
    }

    pub fn reset(&mut self, new_scrimmage_x_mirim: f64) {
        self.down = 1;
        self.advanced_mirins = 0.0;
        self.scrimmage_x_mirim = new_scrimmage_x_mirim;
        self.bonus_phase = BonusPhaseState::Inactive;
    }

    pub fn remaining_mirins_to_target(&self) -> f64 {
        (MINIMUM_ADVANCE_MIRINS_PER_SERIES - self.advanced_mirins).max(0.0)
    }

    pub fn remaining_downs(&self) -> u8 {
        self.max_downs().saturating_sub(self.down)
    }
}
