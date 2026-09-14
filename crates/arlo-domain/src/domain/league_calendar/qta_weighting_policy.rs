use crate::domain::validation::validate_float_range;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct QtaWeightingPolicy {
    home_win_weight: f64,
    away_win_weight: f64,
    home_draw_weight: f64,
    away_draw_weight: f64,
    home_loss_weight: f64,
    away_loss_weight: f64,
}

impl QtaWeightingPolicy {
    pub fn new(
        home_win_weight: f64,
        away_win_weight: f64,
        home_draw_weight: f64,
        away_draw_weight: f64,
        home_loss_weight: f64,
        away_loss_weight: f64,
    ) -> DomainResult<Self> {
        validate_float_range(home_win_weight, -1000.0, 1000.0, "home_win_weight")?;
        validate_float_range(away_win_weight, -1000.0, 1000.0, "away_win_weight")?;
        validate_float_range(home_draw_weight, -1000.0, 1000.0, "home_draw_weight")?;
        validate_float_range(away_draw_weight, -1000.0, 1000.0, "away_draw_weight")?;
        validate_float_range(home_loss_weight, -1000.0, 1000.0, "home_loss_weight")?;
        validate_float_range(away_loss_weight, -1000.0, 1000.0, "away_loss_weight")?;

        Ok(Self {
            home_win_weight,
            away_win_weight,
            home_draw_weight,
            away_draw_weight,
            home_loss_weight,
            away_loss_weight,
        })
    }

    pub fn default_policy() -> Self {
        Self {
            home_win_weight: 1.0,
            away_win_weight: 0.9,
            home_draw_weight: 0.6,
            away_draw_weight: 0.55,
            home_loss_weight: -0.2,
            away_loss_weight: -0.15,
        }
    }

    pub fn home_win_weight(&self) -> f64 {
        self.home_win_weight
    }

    pub fn away_win_weight(&self) -> f64 {
        self.away_win_weight
    }

    pub fn home_draw_weight(&self) -> f64 {
        self.home_draw_weight
    }

    pub fn away_draw_weight(&self) -> f64 {
        self.away_draw_weight
    }

    pub fn home_loss_weight(&self) -> f64 {
        self.home_loss_weight
    }

    pub fn away_loss_weight(&self) -> f64 {
        self.away_loss_weight
    }
}

impl Default for QtaWeightingPolicy {
    fn default() -> Self {
        Self::default_policy()
    }
}
