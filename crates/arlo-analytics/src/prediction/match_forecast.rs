use crate::error::{AnalyticsError, AnalyticsResult};
use crate::power_ranking::PowerRating;
use serde::{Deserialize, Serialize};

pub const MATCH_PREDICTION_MODEL_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ForecastParameters {
    pub home_advantage: f64,
    pub draw_propensity: f64,
    pub temperature: f64,
    pub rating_scale: f64,
}

impl Default for ForecastParameters {
    fn default() -> Self {
        Self {
            home_advantage: 0.0,
            draw_propensity: 0.5,
            temperature: 1.0,
            rating_scale: 400.0,
        }
    }
}

impl ForecastParameters {
    pub fn validate(self) -> AnalyticsResult<()> {
        for (name, value) in [
            ("home_advantage", self.home_advantage),
            ("draw_propensity", self.draw_propensity),
            ("temperature", self.temperature),
            ("rating_scale", self.rating_scale),
        ] {
            if !value.is_finite() {
                return Err(AnalyticsError::NonFinite { field: name.into() });
            }
        }
        if self.draw_propensity <= 0.0 || self.temperature <= 0.0 || self.rating_scale <= 0.0 {
            return Err(AnalyticsError::InvalidData(
                "draw propensity, temperature and rating scale must be positive".into(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MatchPrediction {
    pub home_win: f64,
    pub draw: f64,
    pub away_win: f64,
}

pub fn predict_match(
    home_rating: PowerRating,
    away_rating: PowerRating,
    neutral_venue: bool,
    parameters: ForecastParameters,
) -> AnalyticsResult<MatchPrediction> {
    parameters.validate()?;
    let advantage = if neutral_venue {
        0.0
    } else {
        parameters.home_advantage
    };
    let denominator = parameters.rating_scale * parameters.temperature;
    if !denominator.is_finite() || denominator <= 0.0 {
        return Err(AnalyticsError::NonFinite {
            field: "rating_denominator".into(),
        });
    }
    let difference = (home_rating.value() + advantage - away_rating.value()) / denominator;
    if !difference.is_finite() {
        return Err(AnalyticsError::NonFinite {
            field: "rating_difference".into(),
        });
    }
    let home_log = difference * std::f64::consts::LN_10 / 2.0;
    if !home_log.is_finite() {
        return Err(AnalyticsError::NonFinite {
            field: "home_log_weight".into(),
        });
    }
    let away_log = -home_log;
    let draw_log = parameters.draw_propensity.ln();
    let maximum = home_log.max(away_log).max(draw_log);
    let home = (home_log - maximum).exp();
    let draw = (draw_log - maximum).exp();
    let away = (away_log - maximum).exp();
    let total = home + draw + away;
    Ok(MatchPrediction {
        home_win: home / total,
        draw: draw / total,
        away_win: away / total,
    })
}
