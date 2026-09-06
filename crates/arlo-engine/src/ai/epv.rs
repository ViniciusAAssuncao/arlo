use arlo_domain::sport_constants::{
    FIELD_POINT_VALUE, GOAL_POINT_REQUIRED_DRIVES, GOAL_POINT_VALUE,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct EpvModel;

impl EpvModel {
    pub fn score_value(drives_in_series: u32) -> f64 {
        if drives_in_series >= GOAL_POINT_REQUIRED_DRIVES {
            GOAL_POINT_VALUE as f64
        } else {
            FIELD_POINT_VALUE as f64
        }
    }

    pub fn scoring_probability(
        normalized_x: f64,
        down: u8,
        remaining_advance_mirim: f64,
    ) -> f64 {
        let x = normalized_x.clamp(0.0, 1.0);
        let base_field_factor = 1.0 / (1.0 + (-4.5 * (x - 0.45)).exp());
        let d = down.clamp(1, 4) as f64;
        let down_factor = ((5.0 - d) / 4.0).powf(0.6);
        let distance_factor = 10.0 / (10.0 + remaining_advance_mirim.max(0.0));
        (base_field_factor * down_factor * distance_factor).clamp(0.01, 0.98)
    }

    pub fn turnover_probability(
        normalized_x: f64,
        down: u8,
        remaining_advance_mirim: f64,
    ) -> f64 {
        let x = normalized_x.clamp(0.0, 1.0);
        let p_score = Self::scoring_probability(x, down, remaining_advance_mirim);
        if down >= 4 {
            let dist_factor = 10.0 / (10.0 + remaining_advance_mirim.max(0.0));
            ((1.0 - p_score) * (1.0 - 0.35 * dist_factor)).clamp(0.10, 0.98)
        } else {
            let d_ratio = ((down.max(1) - 1) as f64) / 3.0;
            (0.04 + 0.12 * (1.0 - x) * d_ratio).clamp(0.02, 0.80)
        }
    }

    pub fn opponent_score_value(normalized_x: f64) -> f64 {
        let opp_x = (1.0 - normalized_x).clamp(0.0, 1.0);
        let opp_p_score = Self::scoring_probability(opp_x, 1, 10.0);
        opp_p_score * (FIELD_POINT_VALUE as f64)
    }

    pub fn calculate_epv(
        normalized_x: f64,
        down: u8,
        remaining_advance_mirim: f64,
        drives_in_series: u32,
    ) -> f64 {
        let x = normalized_x.clamp(0.0, 1.0);
        let p_score = Self::scoring_probability(x, down, remaining_advance_mirim);
        let p_turnover = Self::turnover_probability(x, down, remaining_advance_mirim);
        let v_score = Self::score_value(drives_in_series);
        let v_opp = Self::opponent_score_value(x);
        (p_score * v_score) - (p_turnover * v_opp)
    }

    pub fn epv_for_pitch_position(
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
        Self::calculate_epv(norm_x, down, remaining_advance_mirim, drives_in_series)
    }
}