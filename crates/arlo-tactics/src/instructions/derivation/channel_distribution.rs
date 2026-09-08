use crate::instructions::axes::{FlankBias, Width};
use crate::instructions::derivation::channel_distribution_constants::{SIGMA_MAX, SIGMA_MIN};
use arlo_domain::pitch::ArtroPlacement;
use serde::{Deserialize, Serialize};
use std::ops::Index;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ChannelDistribution {
    left: f64,
    central: f64,
    right: f64,
}

impl ChannelDistribution {
    pub fn new(left: f64, central: f64, right: f64) -> Self {
        Self {
            left,
            central,
            right,
        }
    }

    pub fn from_width_and_flank_bias(width: Width, flank_bias: FlankBias) -> Self {
        let width_norm = (width.value() + 1.0) / 2.0;
        let sigma = SIGMA_MIN + width_norm * (SIGMA_MAX - SIGMA_MIN);
        let two_sigma_sq = 2.0 * sigma * sigma;
        let bias = flank_bias.value();

        let w_left = (-((-1.0 - bias).powi(2)) / two_sigma_sq).exp();
        let w_central = (-((0.0 - bias).powi(2)) / two_sigma_sq).exp();
        let w_right = (-((1.0 - bias).powi(2)) / two_sigma_sq).exp();

        let total = w_left + w_central + w_right;

        Self {
            left: w_left / total,
            central: w_central / total,
            right: w_right / total,
        }
    }

    pub fn left(&self) -> f64 {
        self.left
    }

    pub fn central(&self) -> f64 {
        self.central
    }

    pub fn right(&self) -> f64 {
        self.right
    }

    pub fn get(&self, placement: ArtroPlacement) -> f64 {
        self[placement]
    }
}

impl Index<ArtroPlacement> for ChannelDistribution {
    type Output = f64;

    fn index(&self, index: ArtroPlacement) -> &Self::Output {
        match index {
            ArtroPlacement::LeftLateral => &self.left,
            ArtroPlacement::Central => &self.central,
            ArtroPlacement::RightLateral => &self.right,
        }
    }
}

impl Default for ChannelDistribution {
    fn default() -> Self {
        Self::from_width_and_flank_bias(Width::default(), FlankBias::default())
    }
}