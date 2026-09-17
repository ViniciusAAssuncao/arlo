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
        if drives_in_series < GOAL_POINT_REQUIRED_DRIVES {
            return (normalized_x * 0.05).clamp(0.0, 0.05);
        }
        let x = normalized_x.clamp(0.0, 1.0);
        let down_penalty = ((down.clamp(1, 4) - 1) as f64) * 0.10;
        let dist_penalty = (remaining_advance_mirim.max(0.0) / 20.0).clamp(0.0, 0.30);
        (x * 0.80 - down_penalty - dist_penalty).clamp(0.05, 0.95)
    }

    pub fn field_point_probability(
        &self,
        normalized_x: f64,
        drives_in_series: u32,
        down: u8,
        remaining_advance_mirim: f64,
    ) -> f64 {
        if drives_in_series < FIELD_POINT_REQUIRED_DRIVES && normalized_x < 0.40 {
            return 0.0;
        }
        let x = normalized_x.clamp(0.0, 1.0);
        let down_penalty = ((down.clamp(1, 4) - 1) as f64) * 0.08;
        let dist_penalty = (remaining_advance_mirim.max(0.0) / 20.0).clamp(0.0, 0.25);
        (x * 0.65 - down_penalty - dist_penalty).clamp(0.05, 0.90)
    }

    pub fn turnover_probability(
        &self,
        normalized_x: f64,
        down: u8,
        _remaining_advance_mirim: f64,
    ) -> f64 {
        let x = normalized_x.clamp(0.0, 1.0);
        if down >= 4 {
            (0.60 - x * 0.30).clamp(0.15, 0.85)
        } else {
            (0.12 - x * 0.06 + (down as f64) * 0.03).clamp(0.02, 0.40)
        }
    }

    pub fn opponent_epa(&self, normalized_x: f64) -> f64 {
        let opp_x = (1.0 - normalized_x).clamp(0.0, 1.0);
        opp_x * 3.5
    }

    pub fn calculate_epa(
        &self,
        normalized_x: f64,
        down: u8,
        remaining_advance_mirim: f64,
        drives_in_series: u32,
    ) -> f64 {
        let p_goal = self.goal_probability(normalized_x, drives_in_series, down, remaining_advance_mirim);
        let p_field =
            self.field_point_probability(normalized_x, drives_in_series, down, remaining_advance_mirim);
        let p_to = self.turnover_probability(normalized_x, down, remaining_advance_mirim);
        let opp_val = self.opponent_epa(normalized_x);

        p_goal * (GOAL_POINT_VALUE as f64) + p_field * (FIELD_POINT_VALUE as f64)
            - p_to * opp_val
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
        } else {
            FIELD_POINT_VALUE as f64
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
