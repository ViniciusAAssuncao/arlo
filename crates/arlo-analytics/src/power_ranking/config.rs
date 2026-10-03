use crate::error::{AnalyticsError, AnalyticsResult};
use crate::power_ranking::rating::PowerRating;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PowerRankingConfig {
    pub center_rating: f64,
    pub rating_scale: f64,
    pub k_factor: f64,
    pub draw_propensity: f64,
    pub home_advantage: f64,
    pub margin_scale: f64,
    pub max_margin_bonus: f64,
    pub offseason_carryover: f64,
    pub prior_spread: f64,
    pub prior_z_cap: f64,
}

impl Default for PowerRankingConfig {
    fn default() -> Self {
        Self {
            center_rating: 1500.0,
            rating_scale: 400.0,
            k_factor: 28.0,
            draw_propensity: 0.5,
            home_advantage: 0.0,
            margin_scale: 10.0,
            max_margin_bonus: 0.5,
            offseason_carryover: 0.75,
            prior_spread: 60.0,
            prior_z_cap: 2.5,
        }
    }
}

impl PowerRankingConfig {
    pub fn validate(&self) -> AnalyticsResult<()> {
        self.require_finite("center_rating", self.center_rating)?;
        self.require_positive("rating_scale", self.rating_scale)?;
        self.require_positive("k_factor", self.k_factor)?;
        self.require_nonnegative("draw_propensity", self.draw_propensity)?;
        self.require_finite("home_advantage", self.home_advantage)?;
        self.require_positive("margin_scale", self.margin_scale)?;
        self.require_nonnegative("max_margin_bonus", self.max_margin_bonus)?;
        self.require_finite("offseason_carryover", self.offseason_carryover)?;
        self.require_nonnegative("prior_spread", self.prior_spread)?;
        self.require_positive("prior_z_cap", self.prior_z_cap)?;
        if !(0.0..=1.0).contains(&self.offseason_carryover) {
            return Err(AnalyticsError::OutOfRange {
                field: "offseason_carryover".into(),
                min: 0.0,
                max: 1.0,
                value: self.offseason_carryover,
            });
        }
        Ok(())
    }

    pub fn center(&self) -> AnalyticsResult<PowerRating> {
        self.validate()?;
        PowerRating::new(self.center_rating)
    }

    pub fn carry_over(&self, previous: PowerRating) -> AnalyticsResult<PowerRating> {
        self.validate()?;
        PowerRating::new(
            self.center_rating + self.offseason_carryover * (previous.value() - self.center_rating),
        )
    }

    fn require_finite(&self, field: &str, value: f64) -> AnalyticsResult<()> {
        if !value.is_finite() {
            return Err(AnalyticsError::NonFinite {
                field: field.into(),
            });
        }
        Ok(())
    }

    fn require_positive(&self, field: &str, value: f64) -> AnalyticsResult<()> {
        self.require_finite(field, value)?;
        if value <= 0.0 {
            return Err(AnalyticsError::InvalidData(format!(
                "{field} must be positive"
            )));
        }
        Ok(())
    }

    fn require_nonnegative(&self, field: &str, value: f64) -> AnalyticsResult<()> {
        self.require_finite(field, value)?;
        if value < 0.0 {
            return Err(AnalyticsError::InvalidData(format!(
                "{field} must be nonnegative"
            )));
        }
        Ok(())
    }
}
