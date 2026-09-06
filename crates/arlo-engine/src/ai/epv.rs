use arlo_domain::sport_constants::{
    FIELD_POINT_REQUIRED_DRIVES,
    FIELD_POINT_VALUE,
    GOAL_POINT_REQUIRED_DRIVES,
    GOAL_POINT_VALUE,
};
use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DynamicEpvModel {
    offensive_gravity: f64,
}

impl DynamicEpvModel {
    pub fn new(offensive_gravity: f64) -> Self {
        Self {
            offensive_gravity: offensive_gravity.clamp(0.5, 2.5),
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
        remaining_advance_mirim: f64
    ) -> f64 {
        let x = normalized_x.clamp(0.0, 1.0);
        let base_field_factor = 1.0 / (1.0 + (-5.2 * (x - 0.52)).exp());
        let d = down.clamp(1, 4) as f64;
        let down_factor = ((5.0 - d) / 4.0).powf(0.5);
        let distance_factor = 10.0 / (10.0 + remaining_advance_mirim.max(0.0));

        let drive_qualification = if drives_in_series >= GOAL_POINT_REQUIRED_DRIVES {
            1.0
        } else {
            let missing_drives = (GOAL_POINT_REQUIRED_DRIVES - drives_in_series) as f64;
            (1.0 / (1.0 + missing_drives * 2.8)).min(0.2)
        };

        let gravity_effect = 1.0 / (1.0 + (-2.4 * (self.offensive_gravity - 1.0)).exp());
        let gravity_multiplier = 0.45 + 1.1 * gravity_effect;

        (
            base_field_factor *
            down_factor *
            distance_factor *
            drive_qualification *
            gravity_multiplier
        ).clamp(0.0, 0.95)
    }

    pub fn field_point_probability(
        &self,
        normalized_x: f64,
        drives_in_series: u32,
        down: u8,
        remaining_advance_mirim: f64
    ) -> f64 {
        let x = normalized_x.clamp(0.0, 1.0);
        let base_field_factor = 1.0 / (1.0 + (-4.0 * (x - 0.4)).exp());
        let d = down.clamp(1, 4) as f64;
        let down_factor = ((5.0 - d) / 4.0).powf(0.6);
        let distance_factor = 10.0 / (10.0 + remaining_advance_mirim.max(0.0));

        let drive_qualification = if drives_in_series >= FIELD_POINT_REQUIRED_DRIVES {
            1.0
        } else {
            0.35
        };

        let p_goal = self.goal_probability(x, drives_in_series, down, remaining_advance_mirim);
        let raw_p_field = base_field_factor * down_factor * distance_factor * drive_qualification;

        (raw_p_field * (1.0 - p_goal * 0.7)).clamp(0.01, 0.9)
    }

    pub fn turnover_probability(
        &self,
        normalized_x: f64,
        down: u8,
        remaining_advance_mirim: f64
    ) -> f64 {
        let x = normalized_x.clamp(0.0, 1.0);
        let p_scoring_territory = 1.0 / (1.0 + (-4.5 * (x - 0.45)).exp());
        if down >= 4 {
            let dist_factor = 10.0 / (10.0 + remaining_advance_mirim.max(0.0));
            ((1.0 - p_scoring_territory) * (1.0 - 0.35 * dist_factor)).clamp(0.1, 0.98)
        } else {
            let d_ratio = ((down.max(1) - 1) as f64) / 3.0;
            (0.04 + 0.12 * (1.0 - x) * d_ratio).clamp(0.02, 0.8)
        }
    }

    pub fn opponent_epa(&self, normalized_x: f64) -> f64 {
        let opp_x = (1.0 - normalized_x).clamp(0.0, 1.0);
        let opp_model = DynamicEpvModel::new(1.0);
        let p_goal = opp_model.goal_probability(opp_x, 1, 1, 10.0);
        let p_field = opp_model.field_point_probability(opp_x, 1, 1, 10.0);
        p_goal * (GOAL_POINT_VALUE as f64) + p_field * (FIELD_POINT_VALUE as f64)
    }

    pub fn calculate_epa(
        &self,
        normalized_x: f64,
        down: u8,
        remaining_advance_mirim: f64,
        drives_in_series: u32
    ) -> f64 {
        let x = normalized_x.clamp(0.0, 1.0);
        let p_goal = self.goal_probability(x, drives_in_series, down, remaining_advance_mirim);
        let p_field = self.field_point_probability(
            x,
            drives_in_series,
            down,
            remaining_advance_mirim
        );
        let p_turnover = self.turnover_probability(x, down, remaining_advance_mirim);
        let opp_epa = self.opponent_epa(x);

        p_goal * (GOAL_POINT_VALUE as f64) +
            p_field * (FIELD_POINT_VALUE as f64) -
            p_turnover * opp_epa
    }

    pub fn calculate_epv(
        &self,
        normalized_x: f64,
        down: u8,
        remaining_advance_mirim: f64,
        drives_in_series: u32
    ) -> f64 {
        self.calculate_epa(normalized_x, down, remaining_advance_mirim, drives_in_series)
    }

    pub fn epv_for_pitch_position(
        &self,
        pitch_length_meters: f64,
        x_meters: f64,
        down: u8,
        remaining_advance_mirim: f64,
        drives_in_series: u32,
        attacking_positive_x: bool
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
        drives_in_series: u32
    ) -> f64 {
        Self::default().calculate_epa(normalized_x, down, remaining_advance_mirim, drives_in_series)
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
