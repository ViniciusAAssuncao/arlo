use arlo_domain::sport_constants::{
    DEFAULT_BONUS_PHASE_MAX_PLAYS, MAX_CALL_TO_ACTIONS_PER_SERIES, MINIMUM_ADVANCE_MIRINS_PER_SERIES,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SeriesState {
    down: u8,
    advanced_mirins: f64,
    target_advance_mirim: f64,
    drives_in_series: u32,
    is_bonus_phase: bool,
    max_downs: u8,
    bonus_phase_max_plays: u8,
}

impl SeriesState {
    pub fn new(target_advance_mirim: f64, max_downs: u8, bonus_phase_max_plays: u8) -> Self {
        Self {
            down: 1,
            advanced_mirins: 0.0,
            target_advance_mirim,
            drives_in_series: 0,
            is_bonus_phase: false,
            max_downs,
            bonus_phase_max_plays,
        }
    }

    pub fn default_ruleset() -> Self {
        Self::new(
            MINIMUM_ADVANCE_MIRINS_PER_SERIES,
            MAX_CALL_TO_ACTIONS_PER_SERIES as u8,
            DEFAULT_BONUS_PHASE_MAX_PLAYS as u8,
        )
    }

    pub fn down(&self) -> u8 {
        self.down
    }

    pub fn advanced_mirins(&self) -> f64 {
        self.advanced_mirins
    }

    pub fn target_advance_mirim(&self) -> f64 {
        self.target_advance_mirim
    }

    pub fn drives_in_series(&self) -> u32 {
        self.drives_in_series
    }

    pub fn is_bonus_phase(&self) -> bool {
        self.is_bonus_phase
    }

    pub fn effective_max_downs(&self) -> u8 {
        if self.is_bonus_phase {
            self.bonus_phase_max_plays
        } else {
            self.max_downs
        }
    }

    pub fn remaining_mirins_to_target(&self) -> f64 {
        (self.target_advance_mirim - self.advanced_mirins).max(0.0)
    }

    pub fn remaining_downs(&self) -> u8 {
        self.effective_max_downs().saturating_sub(self.down)
    }

    pub fn is_last_down(&self) -> bool {
        self.down >= self.effective_max_downs()
    }

    pub fn has_achieved_target(&self) -> bool {
        self.advanced_mirins >= self.target_advance_mirim
    }

    pub fn should_turnover_on_downs(&self) -> bool {
        if self.is_bonus_phase {
            self.is_last_down()
        } else {
            self.is_last_down() && !self.has_achieved_target()
        }
    }

    pub fn record_advance(&mut self, mirins: f64) {
        self.advanced_mirins += mirins;
    }

    pub fn record_drive(&mut self) {
        self.drives_in_series += 1;
    }

    pub fn set_drives(&mut self, drives: u32) {
        self.drives_in_series = drives;
    }

    pub fn advance_down(&mut self) -> bool {
        if self.down < self.effective_max_downs() {
            self.down += 1;
            true
        } else {
            false
        }
    }

    pub fn set_bonus_phase(&mut self, bonus: bool) {
        self.is_bonus_phase = bonus;
        self.down = 1;
        self.advanced_mirins = 0.0;
    }

    pub fn reset_downs(&mut self) {
        self.down = 1;
        self.advanced_mirins = 0.0;
    }

    pub fn reset(&mut self) {
        self.down = 1;
        self.advanced_mirins = 0.0;
        self.drives_in_series = 0;
        self.is_bonus_phase = false;
    }

    pub fn can_attempt_goal_point(&self, required_drives: u32) -> bool {
        self.drives_in_series >= required_drives && !self.is_bonus_phase
    }

    pub fn can_attempt_field_point(&self, required_drives: u32) -> bool {
        self.drives_in_series >= required_drives
    }
}

impl Default for SeriesState {
    fn default() -> Self {
        Self::default_ruleset()
    }
}