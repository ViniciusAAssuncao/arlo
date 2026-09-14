use crate::domain::validation::{validate_float_range, validate_positive_finite};
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SpaScoringPolicy {
    win_weight: f64,
    draw_weight: f64,
    loss_weight: f64,
    feo_k_factor: f64,
}

impl SpaScoringPolicy {
    pub fn new(
        win_weight: f64,
        draw_weight: f64,
        loss_weight: f64,
        feo_k_factor: f64,
    ) -> DomainResult<Self> {
        validate_float_range(win_weight, 0.0, 1000.0, "win_weight")?;
        validate_float_range(draw_weight, 0.0, 1000.0, "draw_weight")?;
        validate_float_range(loss_weight, 0.0, 1000.0, "loss_weight")?;
        validate_positive_finite(feo_k_factor, "feo_k_factor")?;

        Ok(Self {
            win_weight,
            draw_weight,
            loss_weight,
            feo_k_factor,
        })
    }

    pub fn default_policy() -> Self {
        Self {
            win_weight: 3.0,
            draw_weight: 1.0,
            loss_weight: 0.25,
            feo_k_factor: 5.0,
        }
    }

    pub fn win_weight(&self) -> f64 {
        self.win_weight
    }

    pub fn draw_weight(&self) -> f64 {
        self.draw_weight
    }

    pub fn loss_weight(&self) -> f64 {
        self.loss_weight
    }

    pub fn feo_k_factor(&self) -> f64 {
        self.feo_k_factor
    }
}

impl Default for SpaScoringPolicy {
    fn default() -> Self {
        Self::default_policy()
    }
}
