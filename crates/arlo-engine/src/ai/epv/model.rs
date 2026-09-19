use crate::ai::epv::field_point_probability::calculate_field_point_probability;
use crate::ai::epv::goal_probability::calculate_goal_probability;
use crate::ai::epv::turnover_risk::{calculate_opponent_epa, calculate_turnover_probability};
use arlo_domain::sport_constants::{
    FIELD_POINT_REQUIRED_DRIVES, FIELD_POINT_VALUE, GOAL_POINT_REQUIRED_DRIVES, GOAL_POINT_VALUE,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DynamicEpvModel {
    offensive_gravity: f64,
}

impl DynamicEpvModel {
    pub fn new(offensive_gravity: f64) -> Self {
        Self {
            offensive_gravity: offensive_gravity.max(0.1),
        }
    }

    pub fn offensive_gravity(&self) -> f64 {
        self.offensive_gravity
    }

    pub fn goal_probability(
        &self,
        normalized_x: f64,
        drives_in_series: u32,
        down: u8,
        remaining_advance_mirim: f64,
    ) -> f64 {
        calculate_goal_probability(
            self.offensive_gravity,
            normalized_x,
            drives_in_series,
            down,
            remaining_advance_mirim,
        )
    }

    pub fn field_point_probability(
        &self,
        normalized_x: f64,
        drives_in_series: u32,
        down: u8,
        remaining_advance_mirim: f64,
    ) -> f64 {
        calculate_field_point_probability(
            self.offensive_gravity,
            normalized_x,
            drives_in_series,
            down,
            remaining_advance_mirim,
        )
    }

    pub fn turnover_probability(
        &self,
        normalized_x: f64,
        down: u8,
        remaining_advance_mirim: f64,
    ) -> f64 {
        calculate_turnover_probability(normalized_x, down, remaining_advance_mirim)
    }

    pub fn opponent_epa(&self, normalized_x: f64) -> f64 {
        calculate_opponent_epa(normalized_x)
    }

    pub fn calculate_epa(
        &self,
        normalized_x: f64,
        down: u8,
        remaining_advance_mirim: f64,
        drives_in_series: u32,
    ) -> f64 {
        let p_goal = self.goal_probability(
            normalized_x,
            drives_in_series,
            down,
            remaining_advance_mirim,
        );
        let p_field = self.field_point_probability(
            normalized_x,
            drives_in_series,
            down,
            remaining_advance_mirim,
        );
        let p_to = self.turnover_probability(normalized_x, down, remaining_advance_mirim);
        let opp_val = self.opponent_epa(normalized_x);

        let ev_field = p_field * (FIELD_POINT_VALUE as f64);
        let ev_goal = p_goal * (GOAL_POINT_VALUE as f64);
        let scoring_ev = ev_field.max(ev_goal);

        let drive_progression_value = if drives_in_series < GOAL_POINT_REQUIRED_DRIVES {
            (drives_in_series as f64) * 0.35 * normalized_x.clamp(0.0, 1.0)
        } else {
            0.0
        };

        scoring_ev + drive_progression_value - p_to * opp_val
    }

    pub fn calculate_epv(
        &self,
        normalized_x: f64,
        down: u8,
        remaining_advance_mirim: f64,
        drives_in_series: u32,
    ) -> f64 {
        self.calculate_epa(
            normalized_x,
            down,
            remaining_advance_mirim,
            drives_in_series,
        )
    }

    pub fn epv_for_pitch_position(
        &self,
        pitch_length_meters: f64,
        x_meters: f64,
        down: u8,
        remaining_advance_mirim: f64,
        drives_in_series: u32,
        attacking_positive_x: bool,
    ) -> f64 {
        if pitch_length_meters <= 0.0 {
            return 0.0;
        }
        let norm_x = if attacking_positive_x {
            (x_meters / pitch_length_meters).clamp(0.0, 1.0)
        } else {
            ((pitch_length_meters - x_meters) / pitch_length_meters).clamp(0.0, 1.0)
        };
        self.calculate_epa(norm_x, down, remaining_advance_mirim, drives_in_series)
    }

    pub fn score_value(drives_in_series: u32) -> f64 {
        if drives_in_series >= GOAL_POINT_REQUIRED_DRIVES {
            GOAL_POINT_VALUE as f64
        } else if drives_in_series >= FIELD_POINT_REQUIRED_DRIVES {
            FIELD_POINT_VALUE as f64
        } else {
            0.0
        }
    }

    pub fn static_calculate_epv(
        normalized_x: f64,
        down: u8,
        remaining_advance_mirim: f64,
        drives_in_series: u32,
    ) -> f64 {
        Self::default().calculate_epa(
            normalized_x,
            down,
            remaining_advance_mirim,
            drives_in_series,
        )
    }

    pub fn opponent_score_value(normalized_x: f64) -> f64 {
        Self::default().opponent_epa(normalized_x)
    }
}

impl Default for DynamicEpvModel {
    fn default() -> Self {
        Self::new(1.0)
    }
}

pub type EpvModel = DynamicEpvModel;